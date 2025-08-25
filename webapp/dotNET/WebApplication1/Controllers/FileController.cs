using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Http;

using SampleWopiHandler;
using ProofKeysNamespace;




namespace WebApplication1.Controllers {
  [ApiController]
  [Route("[controller]")]
  public class FileController : Controller {
    [HttpGet("/wopi/files/{fileId}")]
    // This endpoint gets the information of the file, which id value is being used in the endpoint and returns it
    //  to the host machine that is running the webpage from the collabora server. It responds to a GET request at
    //  http://<HOSTNAME>/wopi/files/<fileId>. The minimum amount of information needed to be returned is Name and
    //  Size of the file.
    public ActionResult<FileInfoTemplate> CheckFileInfo() {
      checkProofHeaders();
      return new FileInfoTemplate {
        BaseFileName = "test.txt",
        Size = 11,
        UserId = 1,
        UserCanWrite = true
      };
    }

    [HttpGet("/wopi/files/{fileId}/contents")]
    // This endpoint gets the contents of the file, as this a SDK example the content loaded onto the file origionally
    //  is hardcoded into the system. This is what makes sure when a new page of the SDK is opened it always opens with
    //  the text 'Hello World'. It is called when the GET request is called at http://<HOSTNAME>/wopi/files/<fileId>/contents
    public IActionResult GetFile() {
      checkProofHeaders();
      var fileContent = "Hello World";
      return Content(fileContent);
    }

    [HttpPost("/wopi/files/{fileId}/contents")]
    // This endpoint allows for the files to save to the collabora space. This SDK example outputs in the console, or something
    //  {need to find this}, the body text put bellow then returns the status code 200; meaning it was a success but only if the
    //  body has something in, from being edited. Else it returns a failure code of 404.
    public IActionResult PutFile() {
      checkProofHeaders();
      using (StreamReader stream = new StreamReader(HttpContext.Request.Body)) {
        string? body = stream.ReadToEndAsync().ToString();
        body = "param=somevalue&param2=someothervalue";
        if (body != null) {
          System.Diagnostics.Debug.WriteLine(body);
          return StatusCode(200);
        } else {
          return StatusCode(404);
        }
      }
    }

    /* Uses the unmodified WOPI-Proof check code from Microsoft inside "ProofKeyHelper.cs".
     */
    private void checkProofHeaders() {
      if (ProofKeysStore.helper != null) {
        var r = HttpContext.Request;
        var token = r.Query["access_token"].ToString();
        long.TryParse(r.Headers["X-WOPI-TimeStamp"], out long timestamp100NanoSec);
        var url = $"{r.Scheme}://{r.Host.Value}{r.Path}{r.QueryString}";
        var rpu_prefix = Environment.GetEnvironmentVariable("REVERSE_PROXY_URL_PREFIX");
        if (!String.IsNullOrEmpty(rpu_prefix)) {
          url = $"{rpu_prefix}{r.Path}{r.QueryString}";
          Console.WriteLine("WOPI_PROOF: url behind reverse proxy: " + url);
        }
        var proofSignatureBase64 = r.Headers["X-WOPI-Proof"].ToString();
        var oldProofSignatureBase64 = r.Headers["X-WOPI-ProofOld"].ToString();
        if (token != "" && proofSignatureBase64 != "" && oldProofSignatureBase64 != "") {
          var proofKVI = new ProofKeyValidationInput(token, timestamp100NanoSec,
              url, proofSignatureBase64, oldProofSignatureBase64);
          if (ProofKeysStore.helper.Validate(proofKVI)) {
            Console.WriteLine("WOPI_PROOF: validation successfull");
            // WARNING: On a production server, the request should NOT be answered if validation is NOT successfull.
            //          But this is an example, so a failed is just being logged.
          } else {
            Console.WriteLine("WOPI_PROOF: validation failed");
          }
          return;
        }
        Console.WriteLine("WOPI_PROOF: no validation (no complete set of WOPI-Proof data found)");
      }
    }
  }
}
