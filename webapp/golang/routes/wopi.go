package routes

import (
  "fmt"
  "io"
  "net/http"
  "regexp"
  "strings"
)


/* route request to a function */
func Wopi(w http.ResponseWriter, r *http.Request) {
  rFile := regexp.MustCompile("^/wopi/files/([^/]+)/?(.*)$")
  match := rFile.FindStringSubmatch(r.RequestURI)
  fileId := match[1]
  contents := strings.HasPrefix(match[2], "contents")

  if fileId != "" {
    if !contents {  // just fileId
      if r.Method == "GET" {
        getFile(w, r)
      }
    } else {  // contents
      if r.Method == "GET" {
        getFileContents(w, r)
      } else if r.Method == "POST" {
        postFileContents(w, r)
      }
    }
  }
}


/* load file metadata */
func getFile(w http.ResponseWriter, r *http.Request) {
  fmt.Fprint(w, `{
  "BaseFileName": "test.txt",
  "Size": 11,
  "UserId": 1,
  "UserCanWrite": true,
  "EnableInsertRemoteImage": true
}`)
}


/* load file contents */
func getFileContents(w http.ResponseWriter, r *http.Request) {
  fmt.Fprint(w, "Hello world!")
}


/* save file contents (dummy) */
func postFileContents(w http.ResponseWriter, r *http.Request) {
  _, err := io.ReadAll(r.Body)  // bodyBytes
  if err == nil {
    w.WriteHeader(http.StatusOK)  // 200
  } else {
    fmt.Println("Not possible to get the file content.")
    w.WriteHeader(http.StatusNotFound)  // 404
  }
}
