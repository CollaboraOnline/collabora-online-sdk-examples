using Microsoft.AspNetCore.Mvc;
using System.Xml.XPath;


namespace WebApplication1.Controllers {
  [ApiController]
  [Route("[controller]")]
  public class IndexController : Controller {
    private readonly HttpClient _httpClient;

    public IndexController(IHttpClientFactory httpClientFactory) {
      if (Environment.GetEnvironmentVariable("DISABLE_TLS_CERT_VALIDATION") == "1") {
        // WARNING: Never disable certificate verification on a production server.
        var handler = new HttpClientHandler {
          ServerCertificateCustomValidationCallback =
            HttpClientHandler.DangerousAcceptAnyServerCertificateValidator
        };
        _httpClient = new HttpClient(handler);
      } else {
        _httpClient = httpClientFactory.CreateClient();
      }
    }

    [HttpGet("/collaboraUrl")]
    public async Task<IActionResult> GetCollaboraUrl() {
      var collaboraOnlineHost = Request.Query["server"].ToString();
      var xmlString = await _httpClient.GetStringAsync(collaboraOnlineHost + "/hosting/discovery");
      var xmlStructure = new XPathDocument(new System.IO.StringReader(xmlString));
      var xpath = "/wopi-discovery/net-zone/app[@name='text/plain']/action/@urlsrc";
      var onlineUrl = xmlStructure.CreateNavigator().SelectSingleNode(xpath);
      return Content("{\"url\":\"" + onlineUrl?.Value + "\",\"token\":\"test\"}");
    }
  }
}
