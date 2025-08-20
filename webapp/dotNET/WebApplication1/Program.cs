using System.Security.Cryptography.X509Certificates;


// Uses the Kestrel HTTP server trough the ASP.NET Core middleware.
var builder = WebApplication.CreateBuilder(new WebApplicationOptions {
  Args = args,
  WebRootPath = "../../html"
});

ushort port;
if (!ushort.TryParse(Environment.GetEnvironmentVariable("PORT"), out port)) port = 3000;

var tls_crt = Environment.GetEnvironmentVariable("SSL_CRT_FILE");
var tls_key = Environment.GetEnvironmentVariable("SSL_KEY_FILE");
if (String.IsNullOrEmpty(tls_crt) || String.IsNullOrEmpty(tls_key)) {
  builder.WebHost.ConfigureKestrel(options => {
    options.ListenAnyIP(port);
    if (port != 5123) { options.ListenAnyIP(5123); }
    if (port != 7123) {
      options.ListenAnyIP(7123, listenOptions => { listenOptions.UseHttps(); });
    }
  });
} else {
  builder.WebHost.ConfigureKestrel(options => {
    options.ListenAnyIP(port, listenOptions => {
      listenOptions.UseHttps(X509Certificate2.CreateFromPemFile(tls_crt!, tls_key!));
    });
  });
}

// Add services to the container.
builder.Services.AddControllers().AddJsonOptions(
  opition=>opition.JsonSerializerOptions.PropertyNamingPolicy=null);
// Learn more about configuring Swagger/OpenAPI at https://aka.ms/aspnetcore/swashbuckle
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();
builder.Services.AddHttpClient();  // HTTP client request for discovery.xml

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment()) {
  // hard enabled in: Properties/launchSettings.json
  // for debugging browse to: http(s)://HOSTNAME/swagger/
  app.UseSwagger();
  app.UseSwaggerUI();
}
app.UseDefaultFiles();  // default to index.html
app.UseStaticFiles();   // serve web root
app.MapControllers();

app.Run();
