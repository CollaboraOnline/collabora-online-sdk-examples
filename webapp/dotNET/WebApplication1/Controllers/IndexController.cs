using Microsoft.AspNetCore.Mvc;
using System.Xml.XPath;

using SampleWopiHandler;
using ProofKeysNamespace;


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
      var xmlNav = xmlStructure.CreateNavigator();
      var xpath = "/wopi-discovery/net-zone/app[@name='text/plain']/action/@urlsrc";
      var onlineUrl = xmlNav.SelectSingleNode(xpath);
      xpath = "/wopi-discovery/proof-key/@value";
      var proofKey = xmlNav.SelectSingleNode(xpath)?.ToString() ?? "";
      xpath = "/wopi-discovery/proof-key/@oldvalue";
      var proofOldKey = xmlNav.SelectSingleNode(xpath)?.ToString() ?? "";
      if (proofKey != "" && proofOldKey != "") {
        // ATTENTION
        // Will only work with one WOPI-Office-Server at a time.
        // Different WOPI-Office-Servers with different keys won't work, because this code can store
        // only one new-old key pair for one server at a time.
        ProofKeysStore.helper = new ProofKeysHelper(new KeyInfo(proofKey, "", ""), new KeyInfo(proofOldKey, "", ""));
      }
      return Content("{\"url\":\"" + onlineUrl?.Value + "\",\"token\":\"test\"}");
    }
  }
}
