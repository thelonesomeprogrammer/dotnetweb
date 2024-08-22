using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Identity.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddAuthentication().AddCookie(IdentityConstants.ApplicationScheme);
builder.Services.AddAuthorizationBuilder();

var connectionString = builder.Configuration.GetConnectionString("DefaultConnection");
builder.Services.AddDbContext<DbContext>(options =>
    options.UseMySql(connectionString, ServerVersion.AutoDetect(connectionString)));

builder.Services.AddIdentityCore<User>().AddEntityFrameworkStores<DbContext>().AddApiEndpoints();


var app = builder.Build();


app.MapIdentityApi<User>();

app.UseStaticFiles();
app.MapFallbackToFile("/index.html");
app.Run();


class User : IdentityUser{}

class DbContext : IdentityDbContext<User> {
  public DbContext(DbContextOptions<DbContext> options) : base(options){}
}
