using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Http;

namespace WebApplication1.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class FileControllers : Controller
    {
        private readonly ILogger<FileControllers> _logger;

        public FileControllers(ILogger<FileControllers> logger)
        {
            _logger = logger;
        }
        [HttpGet("/wopi/files/{fileId}")]
        public IEnumerable<FileInfoTemplate> CheckFileInfo()
        {
            //_logger.LogInformation("file id: " + /*req.params.fileId*/);
            return Enumerable.Range(0, 1).Select(index => new FileInfoTemplate
            {
                BaseFileName = "test.txt",
                Size = 11,
                UserId = 1,
                UserCanWrite = true
            })
            .ToArray();
        }
        [HttpGet("/wopi/files/{fileId}/contents")]
        public IActionResult GetFile()
        {
            var fileContent = "Hello World";
            return base.Content(fileContent);
        }
        //[HttpPost("/wopi/files/{fileId}/contents")]
        //public IActionResult PutFile()
        //{
        //    _logger.LogInformation("wopi PutFile endpoint");
        //    if (HttpRequest.Body)
        //    {
        //        System.Diagnostics.Debug.WriteLine(HttpRequest.Body);
        //        _logger.LogInformation(HttpRequest.Body.tostring());
        //        return StatusCode(200);
        //    }
        //    else
        //    {
        //        _logger.LogInformation("Not possible to get the file content.");
        //        return StatusCode(404);
        //    }
        //}
    }
}
