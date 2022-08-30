using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Http;

namespace WebApplication1.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class FileControllers : Controller
    {
        // .... NEED TO FIGURE OUT WHAT DOES
        private readonly ILogger<FileControllers> _logger;

        public FileControllers(ILogger<FileControllers> logger)
        {
            _logger = logger;
        }

        [HttpGet("/wopi/files/{fileId}")]
        // This endpoint gets the information of the file, which id value is being used in the endpoint and returns it 
        //  to the host machiene that is running the webpage from the collabora server. 
        public ActionResult<FileInfoTemplate> CheckFileInfo()
        {
            //_logger.LogInformation("file id: " + /*req.params.fileId*/); *This is still in progress but is meant to write the information to the logger.*
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
        //  the text 'Hello World'. 
        public IActionResult GetFile()
        {
            var fileContent = "Hello World";
            return base.Content(fileContent);
        }

  //// Post endpoint to be made to work 

        //[HttpPost("/wopi/files/{fileId}/contents")]
        //public IActionResult PutFile()
        //{
        //    using (StreamReader stream = new StreamReader(HttpContext.Request.Body))
        //    {
        //        string body = stream.ReadToEnd();
        //        // body = "param=somevalue&param2=someothervalue"

        //        _logger.LogInformation("wopi PutFile endpoint");
        //        if (body!=null)
        //        {
        //            System.Diagnostics.Debug.WriteLine(body);
        //            _logger.LogInformation(body);
        //            return StatusCode(200);
        //        }
        //        else
        //        {
        //            _logger.LogInformation("Not possible to get the file content.");
        //            return StatusCode(404);
        //        }
        //    }
        //}
    }
}
