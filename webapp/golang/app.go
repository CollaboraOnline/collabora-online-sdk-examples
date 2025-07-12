package main

import (
  "fmt"
  "net/http"
  "os"
  "strconv"
  "strings"
)

import "CollaboraOnlineIntegrationExample/routes"


func main() {
  port, portErr := strconv.Atoi(os.Getenv("PORT"))
  if portErr != nil || port < 1 || port > 65535 { port = 3000 }
  listenAddr := fmt.Sprintf("127.0.0.1:%d", port);

  tlsCrt := os.Getenv("SSL_CRT_FILE")
  tlsKey := os.Getenv("SSL_KEY_FILE")
  hasTls := tlsCrt != "" && tlsKey != ""
  proto := "http"
  if hasTls { proto = "https" }

  fmt.Println("Listening on " + proto + "://" + listenAddr + "/")
  fmt.Println("Don't use \"localhost\" when listening only to IPv4 127.0.0.1. coolwsd might resolve it to IPv6 [::1].\n")

  http.HandleFunc("/", func (w http.ResponseWriter, r *http.Request) {
    fmt.Printf("%s: %s\n", r.Method, r.RequestURI)
    if strings.HasPrefix(r.RequestURI, "/wopi") {
      routes.Wopi(w, r)
    } else if strings.HasPrefix(r.RequestURI, "/collaboraUrl") {
      routes.CollaboraUrl(w, r, os.Getenv("DISABLE_TLS_CERT_VALIDATION") == "1")
    } else {
      routes.Index(w, r)
    }
  })
  var err error;
  if hasTls {
    err = http.ListenAndServeTLS(listenAddr, tlsCrt, tlsKey, nil)
  } else {
    err = http.ListenAndServe(listenAddr, nil)
  }
  fmt.Fprintln(os.Stderr, err)
}
