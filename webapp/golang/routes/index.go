package routes

import (
  "crypto/tls"
  "encoding/xml"
  "errors"
  "fmt"
  "io"
  "net/http"
  "os"
)


func Index(w http.ResponseWriter, r *http.Request) {
  if r.Method == "GET" {
    http.FileServer(http.Dir("../html")).ServeHTTP(w, r)
  }
}


type xmlWopiDiscovery struct {
  XMLName xml.Name `xml:"wopi-discovery"`
  NetZoneAry []xmlNetZone `xml:"net-zone"`
}
type xmlNetZone struct { AppAry []xmlApp `xml:"app"` }
type xmlApp struct {
  Name string `xml:"name,attr"`
  ActionAry []xmlAction `xml:"action"`
}
type xmlAction struct { UrlSrc string `xml:"urlsrc,attr"` }

/* Discover full URL at the collabora server.
 * example call : GET /collaboraUrl?server=http://127.0.0.1:9980 */
func CollaboraUrl(w http.ResponseWriter, r *http.Request, insecureSkipVerify bool) {
  if r.Method == "GET" {
    var err error = nil;
    defer func() {
      if r := recover(); r != nil {
        fmt.Fprintln(os.Stderr, err);
      }
    }()
    collaboraOnlineHost := r.URL.Query().Get("server")
    // WARNING: Never disable certificate verification on a production server.
    client := &http.Client { Transport: &http.Transport{
      TLSClientConfig: &tls.Config{InsecureSkipVerify: insecureSkipVerify},
    }}
    clientResponse, err := client.Get(collaboraOnlineHost + "/hosting/discovery")
    xmlBytes, err := io.ReadAll(clientResponse.Body)
    var wopiDiscovery xmlWopiDiscovery
    xml.Unmarshal(xmlBytes, &wopiDiscovery)
    err = errors.New("No valid discovery.xml received from the Collabora Online Server.");
    // xpath: /wopi-discovery/net-zone/app[@name='text/plain']/action/@urlsrc
    appAry := wopiDiscovery.NetZoneAry[0].AppAry
    var onlineUrl string
    for i := len(appAry)-1; i > -1; i-- {
      appElem := appAry[i]
      if appElem.Name == "text/plain" {
        onlineUrl = appElem.ActionAry[0].UrlSrc
        i = -1
      }
    }
    err = nil;
    fmt.Fprint(w, `{"url":"` + onlineUrl + `","token":"test"}`)
  }
}
