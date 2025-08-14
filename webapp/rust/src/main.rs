use std::{
  env,
  fs,
  io::{BufReader, prelude::*},
  net::TcpListener
};
use openssl::ssl::{SslMethod, SslAcceptor, SslFiletype};
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
            method: method_args.next().unwrap_or("").to_string(),
            url: method_args.next().unwrap_or("").to_string(),
            response: None
          };

          route_wopi(&mut d);
          route_index(&mut d);
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


fn route_wopi(d: &mut Data) {
  if d.response.is_none() && d.url.starts_with("/wopi/files/") {
    let sub_url = &d.url[12..d.url.len()];
    let file_id_end = sub_url.find('/').unwrap_or(sub_url.len());
    let file_id = &sub_url[0..file_id_end];
    let contents = &sub_url[file_id_end..sub_url.len()].starts_with("/contents?");
    if file_id != "" {
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
fn route_index(d: &mut Data) {
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
      let xpath = "/wopi-discovery/net-zone/app[@name='text/plain']/action/@urlsrc";
      let document = package.as_document();
      if let Ok(online_url) = evaluate_xpath(&document, xpath) {
        d.response = Some(
          format!("{{\"url\":\"{}\",\"token\":\"test\"}}",
          online_url.string()));
      }
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


struct Data {
  method: String,
  url: String,
  response: Option<String>
}


const GET_FILE_JSON: &str = r#"{
  "BaseFileName": "test.txt",
  "Size": 11,
  "UserId": 1,
  "UserCanWrite": true,
  "EnableInsertRemoteImage": true
}"#;
