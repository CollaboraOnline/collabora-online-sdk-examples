// This is a basic class as to the infomation needed to be held about a document.
// It is used in the GET request http://<HOSTNAME>/wopi/files/<fileId> endpoint.
namespace WebApplication1 {
  public class FileInfoTemplate {
    public string? BaseFileName { get; set; }
    public int Size { get; set; }
    public int UserId { get; set; }
    public bool UserCanWrite { get; set; }
  }
}
