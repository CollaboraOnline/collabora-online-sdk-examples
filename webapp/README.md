# Content

- [dotNET](dotNET): .NET example
- [golang](golang): Go example
- [nodejs](nodejs): NodeJS example
- [nodejs-esm](nodejs-esm): same as the nodejs example but using ESM
- [php](php): PHP example
- [python](python): Python example
- [reactjs](reactjs): ReactJS example
- [rust](rust): Rust example
- html: The frontend part used by some other examples. This is NOT an example itself.

# Running the WebApp Examples

These examples demonstrate how to [integrate Collabora Online](https://sdk.collaboraonline.com/docs/How_to_integrate.html) via the [WOPI API](https://en.wikipedia.org/wiki/Web_Application_Open_Platform_Interface) into other applications. See the example folders for specific instructions.

The example will run (in) an HTTP server. To then make use of it, you additionally need a Collabora Online HTTP Server. And the example server and the Collabora Online Server need to be able to reach each other to work. The Collabora Online Server will contact the example server by the same address you use to open the example's web page.

Serve via HTTPS/TLS/SSL to make web browsers allow [secure context](https://developer.mozilla.org/docs/Web/Security/Secure_Contexts) actions like using the clipboard. Set the environment variables `SSL_CRT_FILE=crt.pem` and `SSL_KEY_FILE=key.pem` to enable HTTPS when running the examples. When enabling HTTPS on the example, web browsers require to also use HTTPS for the Collabora Online Server URL and visa versa.

See also: https://sdk.collaboraonline.com/docs/examples.html

## Collabora Online: Run it locally

For running things locally you might want to create self-signed X.509 certificates using [minica](https://github.com/jsha/minica) or running:  
`openssl req -x509 -newkey rsa:4096 -keyout key.pem -out crt.pem -days 365 -nodes -subj "/CN=127.0.0.1"

**DO NOT DISABLE CERTIFICATE VERIFICATION IN PRODUCTION!**

To make the example server accept a self-signed certificate used by the Collabora Online Server, set the environment variable `DISABLE_TLS_CERT_VALIDATION=1`.

To make a local Collabora Online Server accept a self-signed certificate used by the example server, ensure the file `coolwsd.xml` has set `config -> ssl -> ssl_verification` to `false`.

### Pre-Build Collabora Online Development Edition Download

see: https://www.collaboraonline.com/code/#learnmorecode

SSL options for the container (Docker) image:
```
--o:ssl.key_file_path="key.pem" \
--o:ssl.cert_file_path="crt.pem" \
--o:ssl.ca_file_path="ca-chain.cert.pem"
```

### Build from Source

You might [compile the Collabora Online Server from source](https://collaboraonline.github.io/post/build-code/) and run it locally. When doing this, you won't need to make the example server publicly reachable.

To use HTTPS do NOT disable SSL when building Collabora Online.

## Collabora Online: Use public Demo Server

When running an example, you might enter `https://demo.eu.collaboraonline.com` or `https://demo.us.collaboraonline.com` as Collabora Online Server in the example's web form.  
This requires serving the example on a public address with a valid certificate for HTTPS. **Self-Signed certificates won't work!**

E.g. serve the example publicly at `https://yourdomain.example.org/`. And (**important**) use `https://yourdomain.example.org/` to open the example in your web browser. Do NOT open the example using `https://127.0.0.1/` or `https://SOMETHING_INTERNAL/`, because the address from your browsers URL bar will be used by the Collabora Online Server to to callbacks on the example.

You might either run the example directly via HTTPS with a valid certificate using the environment variables `SSL_CRT_FILE` and `SSL_KEY_FILE`. Or you run it with plain HTTP and put it behind a [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) like [Apache](https://httpd.apache.org/docs/current/howto/reverse_proxy.html) to enable HTTPS.

Do NOT put the example behind any kind of authentication like HTTP Basic Auth, because the Collabora Online Server need to do callbacks on the example server. For additional security the `demo.*.collaboraonline.com` servers sign their data using [WOPI proof](https://sdk.collaboraonline.com/docs/advanced_integration.html#wopi-proof), so your code might verify these signatures.

## Miscellaneous

Most examples will accept these environment variables:
- `SSL_CRT_FILE` & `SSL_KEY_FILE`: Serve with HTTPS.
  - Without HTTP will be used and things like clipboard support will be disabled by your browser.
- `DISABLE_TLS_CERT_VALIDATION=1`: Don't validate the Collabora Server's HTTPS certificate. NOT FOR PRODUCTION!
- `PORT`: Most examples use TCP port `3000` as default.

# Setup Schematic

These example apps implement the WOPI Host as a single server, taking on the role of a frontend as well as a document storage backend. In a real application, the WOPI Host might be split into different servers for the frontend and document storage backend.

Simplified sequence diagram:

```
                        example HTTP            Collabora Online
                         App Server            HTTP Office Server
web browser             (WOPI Host)           (WOPI Editor/Client)
-----------             ------------          --------------------
     |                        |                         |
     |  calls: frontend_addr  |                         |
     |  data: collabora_addr  |                         |
     | ---------------------> |                         |
     |                        |                         |
     |              ----------------------              |
     |              | doc_stor_addr=     |              |
     |              | frontend_addr      |              |
     |              | # addr of this srv |              |
     |              ----------------------              |
     |                        |                         |
     |                        |  calls: collabora_addr  |
     |                        |  data: doc_stor_addr    |
     |                        | ----------------------> |
     |                        |                         |
                ...                      ...
     |                        |                         |
     |                        |   calls: doc_stor_addr  |
     |                        | <---------------------- |
```
