using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Http;

namespace WebApplication1.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class FileControllers : Controller
    {
        [HttpGet("/wopi/files/{fileId}")]
        // This endpoint gets the information of the file, which id value is being used in the endpoint and returns it 
        //  to the host machiene that is running the webpage from the collabora server. It responds to a GET request at
        //  http://<HOSTNAME>/wopi/files/<fileId>. The minimum amount of information needed to be returned is Name and 
        //  Size of the file. 
        public ActionResult<FileInfoTemplate> CheckFileInfo()
        { 
            return new FileInfoTemplate
            {
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
        public IActionResult GetFile()
        {
            var fileContent = "Hello World";
            return base.Content(fileContent);
        }

        [HttpPost("/wopi/files/{fileId}/contents")]
        // This endpoint allows for the files to save to the collabora space. This SDK example outputs in the console, or something 
        //  {need to find this}, the body text put bellow then returns the status code 200; meaning it was a success but only if the 
        //  body has something in, from being edited. Else it returns a failure code of 404.  
        public IActionResult PutFile()
        {
            using (StreamReader stream = new StreamReader(HttpContext.Request.Body))
            {
                string body = stream.ReadToEndAsync().ToString();
                body = "param=somevalue&param2=someothervalue";

                if (body != null)
                {
                    System.Diagnostics.Debug.WriteLine(body);
                    return StatusCode(200);
                }
                else
                {
                    return StatusCode(404);
                }
            }
        }

    }
}
