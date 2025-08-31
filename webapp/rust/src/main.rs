use std::{
  env,
  fs,
  io::{BufReader, prelude::*},
  net::TcpListener
};

use openssl::{base64, bn::BigNum, error::ErrorStack, hash::MessageDigest, pkey,
  pkey::PKey, rsa::Rsa, sign::Verifier};
use openssl::ssl::{SslMethod, SslAcceptor, SslFiletype};
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;
use std::io::{Read, Write};
trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

use curl::easy::Easy;

use sxd_document::parser;
use sxd_xpath::evaluate_xpath;


// TODO: print all errors from unexpected behaviour!
fn main() {
  let port = env::var("PORT").ok()
    .and_then(|s| s.parse::<u16>().ok()).unwrap_or(3000);

  let addr_string = format!("127.0.0.1:{}", port);
  let addr = addr_string.as_str();

  let tls_crt = env::var("SSL_CRT_FILE").unwrap_or_default();
  let tls_key = env::var("SSL_KEY_FILE").unwrap_or_default();
  let mut acceptor: Option<Arc<SslAcceptor>> = None;
  let mut proto = "http";
  if !tls_crt.is_empty() && !tls_key.is_empty() {
    proto = "https";
    let mut new_acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    new_acceptor.set_private_key_file(tls_key, SslFiletype::PEM).unwrap();
    new_acceptor.set_certificate_chain_file(tls_crt).unwrap();
    new_acceptor.check_private_key().unwrap();
    acceptor = Some(Arc::new(new_acceptor.build()));
  }

  let listener = TcpListener::bind(addr).unwrap();
  println!("Listening on {}://{}/", proto, addr);
  println!("Don't use \"localhost\" when listening only to IPv4 127.0.0.1. coolwsd might resolve it to IPv6 [::1].\n");


  // ATTENTION
  // Will only work with one WOPI-Office-Server at a time.
  // Different WOPI-Office-Servers with different keys won't work, because this code can store
  // only one new-old key pair for one server at a time.
  let mut keys = Keys::default();  // WOPI Proof

  for w_stream in listener.incoming() {
    println!("connection received: {:?}", w_stream);
    if let Ok(t_stream) = w_stream {
      let mut stream: Box<dyn ReadWrite> = Box::new(&t_stream);
      let mut ssl_err = false;
      if let Some(acceptor) = acceptor.as_ref() {
        if let Ok(s_stream) = acceptor.clone().accept(&t_stream) {
          stream = Box::new(s_stream);
        } else {
          ssl_err = true;
        }
      }

      if !ssl_err {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut lines = buf_reader.by_ref().lines();
        if let Some(Ok(method_line)) = lines.next() {
          println!("HTTP method line: {:?}", method_line);  // example: GET /index.html HTTP/1.1
          let mut method_args = method_line.split(" ");
          let mut d = Data {
            proto: proto.to_string(),
            method: method_args.next().unwrap_or("").to_string(),
            url: method_args.next().unwrap_or("").to_string(),
            headers: HashMap::new(),
            response: None
          };

          while let Some(line) = lines.next() {
            let line_str = line.unwrap_or_default();
            if line_str == "" { break; }
            let header: Vec<&str> = line_str.splitn(2, ": ").collect();
            if header.len() == 2 {
              d.headers.insert(header[0].to_string(), header[1].to_string());
            }
          }

          route_wopi(&mut d, &keys);
          route_index(&mut d, &mut keys);
          route_static(&mut d);

          // send response
          if let Some(ref u_response) = d.response {
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{u_response}");
            _ = stream.write_all(response.as_bytes());  // ignore err
          }
        }
      }
    }
  }
}


fn route_wopi(d: &mut Data, keys: & Keys) {
  if d.response.is_none() && d.url.starts_with("/wopi/files/") {
    let sub_url = &d.url[12..d.url.len()];
    let file_id_end = sub_url.find('/').unwrap_or(sub_url.len());
    let file_id = &sub_url[0..file_id_end];
    let contents = &sub_url[file_id_end..sub_url.len()].starts_with("/contents?");
    if file_id != "" {
      check_proof(d, keys);
      if !contents {  // just file_id => load file metadata
        if d.method == "GET" { d.response = Some(GET_FILE_JSON.to_string()) }
      } else {  // contents
        if d.method == "GET" {  // load file contents
          d.response = Some("Hello world!".to_string());
        } else if d.method == "POST" {  // save file contents (dummy)
          d.response = Some("".to_string());
        }
      }
    }
  }
}


/* Discover full URL at the collabora server.
 * example call : GET /collaboraUrl?server=http://127.0.0.1:9980 */
fn route_index(d: &mut Data, keys: &mut Keys) {
  // example: GET /collaboraUrl?server=http://127.0.0.1:9980
  if d.response.is_none() && d.method == "GET" && d.url.starts_with("/collaboraUrl?server=") {
    let co_host = &d.url[21..d.url.len()].split(" ").next().unwrap_or("");
    let mut data = Vec::new();
    {
      let mut handle = Easy::new();
      if env::var("DISABLE_TLS_CERT_VALIDATION").unwrap_or_default() == "1" {
        // WARNING: Never disable certificate verification on a production server.
        _ = handle.ssl_verify_host(false);
        _ = handle.ssl_verify_peer(false);
      }
      _ = handle.url(format!("{}/hosting/discovery", co_host).as_str());
      let mut transfer = handle.transfer();
      _ = transfer.write_function(|new_data| {
        data.extend_from_slice(new_data);
        Ok(new_data.len())
      });
      _ = transfer.perform();
    }
    let xml_str = std::str::from_utf8(&data).unwrap_or("");
    if let Ok(package) = parser::parse(xml_str) {
      let document = package.as_document();
      let xpath = "/wopi-discovery/net-zone/app[@name='text/plain']/action/@urlsrc";
      if let Ok(online_url) = evaluate_xpath(&document, xpath) {
        d.response = Some(
          format!("{{\"url\":\"{}\",\"token\":\"test\"}}",
          online_url.string()));
      }
      keys.key = key_from_xml("", document);
      keys.old_key = key_from_xml("old", document);
    }
  }
}


/* use this function last */
fn route_static(d: &mut Data) {
  if d.response.is_none() && d.method == "GET" && !d.url.contains("../") {
    let mut path = d.url.clone();
    if path.ends_with("/") { path = format!("{path}/index.html") }
    d.response = Some(fs::read_to_string(format!("../html{path}")).unwrap_or_default());
  }
}




// WOPI-Proof code below

fn key_from_xml(name_prfx: &str, document: sxd_document::dom::Document)
    -> Option<pkey::PKey<pkey::Public>> {
  let xpath = "/wopi-discovery/proof-key/@";
  // Many servers don't provide key data in the discovery.xml, so fail silently.
  let csp_b64 = evaluate_xpath(&document, format!("{}{}value", xpath, name_prfx).as_str()).ok()?;
  let mod_b64 = evaluate_xpath(&document, format!("{}{}modulus", xpath, name_prfx).as_str()).ok()?;
  let exp_b64 = evaluate_xpath(&document, format!("{}{}exponent", xpath, name_prfx).as_str()).ok()?;

  // decode CSP key format
  //    0:  06 02 00 00  # bType, bVersion, reserved, reserved
  //    4:  00 A4 00 00  # aiKeyAlg (RSA key), aiKeyAlg, aiKeyAlg, aiKeyAlg
  //    8:  52 53 41 31  # MAGIC (ascii: RSA1 => public key)
  //   12:  00 0C 00 00  # KEY_LENGTH, little endian, for example 3072
  //   16:  01 00 01 00  # EXPONENT, little endian
  //   20:  ...3072 bytes modulus
  let mut csp_mod: Vec<u8> = Vec::new();
  let mut csp_exp = 0;
  if let Ok(csp_data) = base64::decode_block(csp_b64.string().as_str()) {
    if  csp_data.len() > 20 &&
        csp_data[0] == 0x06 &&
        csp_data[1] == 0x02 &&
        csp_data[4..8] == [0x00, 0xA4, 0x00, 0x00] &&
        csp_data[8..12] == [0x52, 0x53, 0x41, 0x31] {
      let mod_len = (u32::from_le_bytes(csp_data[12..16].try_into().unwrap()) / 8) as usize;
      if csp_data.len() >= 20 + mod_len {
        csp_mod = csp_data[20..20 + (mod_len as usize)].to_vec();
        csp_mod.reverse();
        csp_exp = u32::from_le_bytes(csp_data[16..20].try_into().unwrap());  // success indication
      }
    }
  }
  if csp_exp == 0 {
    println!("WOPI_PROOF: Error parsing CSP key value from discovery XML!");
  }

  let result = (|| -> Result<pkey::PKey<pkey::Public>, Box<dyn std::error::Error>> {
    // But if key data is found, parsing errors are of interest and will be printed.
    let mod_bytes = base64::decode_block(mod_b64.string().as_str()).map_err(|_| ErrorStack::get())?;
    let exp_bytes = base64::decode_block(exp_b64.string().as_str()).map_err(|_| ErrorStack::get())?;
    let mod_bn = BigNum::from_slice(&mod_bytes)?;
    let exp_bn = BigNum::from_slice(&exp_bytes)?;
    // CSP key value is redundant to the modulus+exponent information.
    // Compare them, to ensure the server implements this correctly.
    if  csp_exp != 0 &&
        (mod_bn != BigNum::from_slice(&csp_mod)? || exp_bn != BigNum::from_u32(csp_exp)?) {
      println!("WOPI_PROOF: CSP format key and modulus+exponent key differ! (exponent:{} csp_exponent:{})", exp_bn, csp_exp);
      println!("WOPI_PROOF: Continuing using modulus+exponent key.");
    }
    Ok(PKey::from_rsa(Rsa::from_public_components(mod_bn, exp_bn)?)?)
  })();
  if let Err(ref e) = result { println!("{}", e); }
  return result.ok();
}


fn check_proof(d: &Data, keys: &Keys) {
  if let (Some(key), Some(old_key)) = (&keys.key, &keys.old_key) {
    let mut incomplete = true;
    let token = Regex::new(r"\?(|.+&)access_token=([^&]*)")
      .unwrap().captures(d.url.as_str()).and_then(|c| Some(c[2].to_string()));
    let time_str = d.headers.get("X-WOPI-TimeStamp");  // 100 nano seconds
    let host = d.headers.get("Host");
    let sig_b64 = d.headers.get("X-WOPI-Proof");
    let old_sig_b64 = d.headers.get("X-WOPI-ProofOld");
    if let (Some(token), Some(time_str), Some(host), Some(sig_b64), Some(old_sig_b64)) =
        (token, time_str, host, sig_b64, old_sig_b64) {
      let rpu_prefix = env::var("REVERSE_PROXY_URL_PREFIX").unwrap_or_default();
      let mut url = format!("{}://{}{}", d.proto, host, d.url);
      if !rpu_prefix.is_empty() {
        url = format!("{}{}", rpu_prefix, d.url);
        println!("WOPI_PROOF: url behind reverse proxy: {}", url);
      };
      if let (Ok(time_long), Ok(sig), Ok(old_sig)) = (
            time_str.parse::<u64>(),
            base64::decode_block(sig_b64.as_str()),
            base64::decode_block(old_sig_b64.as_str())
          ) {
        if let (Ok(token_len), Ok(url_len)) =
            (u32::try_from(token.len()), u32::try_from(url.len())) {
          let mut sig_data: Vec<u8> = Vec::new();
          sig_data.extend_from_slice(&token_len.to_be_bytes());
          sig_data.extend_from_slice(token.as_bytes());
          sig_data.extend_from_slice(&url_len.to_be_bytes());
          sig_data.extend_from_slice(url.to_uppercase().as_bytes());
          sig_data.extend_from_slice(&(8 as u32).to_be_bytes());  // timestamp binary length (always "8")
          sig_data.extend_from_slice(&time_long.to_be_bytes());
          incomplete = false;
          if  verify_proof(key, &sig, &sig_data) ||
              verify_proof(key, &old_sig, &sig_data) ||
              verify_proof(old_key, &sig, &sig_data) {
            println!("WOPI_PROOF: validation successfull");
          } else {
            println!("WOPI_PROOF: validation failed");
          }
        }
      }
    }
    if incomplete {
      println!("WOPI_PROOF: no validation (no complete set of WOPI-Proof data found)");
    }
  }
}


fn verify_proof(key: &pkey::PKey<pkey::Public>, sig: &Vec<u8>, data: &Vec<u8>) -> bool {
  if let Ok(mut verifier) = Verifier::new(MessageDigest::sha256(), key) {
    verifier.update(&data[..]).ok();
    return verifier.verify(&sig).unwrap_or(false);
  }
  return false;
}


#[derive(Default)]
struct Keys {
  key: Option<pkey::PKey<pkey::Public>>,
  old_key: Option<pkey::PKey<pkey::Public>>
}




// auxiliary stuff below

struct Data {
  proto: String,
  method: String,
  url: String,  // without host part
  headers: HashMap<String, String>,
  response: Option<String>
}


const GET_FILE_JSON: &str = r#"{
  "BaseFileName": "test.txt",
  "Size": 11,
  "UserId": 1,
  "UserCanWrite": true,
  "EnableInsertRemoteImage": true
}"#;
