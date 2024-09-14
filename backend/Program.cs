using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Identity.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;
using System.Net.WebSockets;
using System.Security.Claims;


var builder = WebApplication.CreateBuilder(args);

builder.Services.AddAuthentication().AddCookie(IdentityConstants.ApplicationScheme);
builder.Services.AddAuthorizationBuilder();

var connectionString = builder.Configuration.GetConnectionString("DefaultConnection");
builder.Services.AddDbContext<DbContext>(options =>
    options.UseMySql(connectionString, ServerVersion.AutoDetect(connectionString)));

builder.Services.AddIdentityCore<User>().AddEntityFrameworkStores<DbContext>().AddApiEndpoints();


var WebSocketOptions = new WebSocketOptions
{
  KeepAliveInterval = TimeSpan.FromMinutes(2)
};

List<Player> codes = [];

var app = builder.Build();

app.MapIdentityApi<User>();
app.UseStaticFiles();
app.UseAuthorization();
app.UseAuthentication();
app.MapFallbackToFile("/index.html");
app.UseWebSockets(WebSocketOptions);

app.Use(async (context, next) =>
{
    if (context.Request.Path == "/sock/kryds")
    {
        var user = context.User;
        if (context.WebSockets.IsWebSocketRequest && (user?.Identity?.IsAuthenticated ?? false))
        {
            using var webSocket = await context.WebSockets.AcceptWebSocketAsync();
            var buffer = new byte[1024];
            var receiveResult = await webSocket.ReceiveAsync(new ArraySegment<byte>(buffer), CancellationToken.None);
            var code = buffer[0..8];
            var userId = user.FindFirst(ClaimTypes.NameIdentifier)?.Value;
            if (code[0..1].ToString() == "c:"){
              var join = code[2..8];
              var p1 = new Player(userId,webSocket,join);
              if (codes.Exists(x => x.kode == p1.kode)){
                Player p2 = codes.Find(x => x.kode == p1.kode);
                codes.Remove(p2);
                var game = new Game(p1,p2);
                game.loop();
              } else {
                codes.Add(p1);
              }
            }
        }
        else
        {
            context.Response.StatusCode = StatusCodes.Status400BadRequest;
        }
    }
    else
    {
        await next(context);
    }

});

app.Run();


class User : IdentityUser{}

class DbContext : IdentityDbContext<User> {
  public DbContext(DbContextOptions<DbContext> options) : base(options){}
  public DbSet<Game>? Games { get; set; }
}

class Game
{
  string id { get; set; }
  string turn { get; set; }
  int[,] state { get; set; }
  int actviebord { get; set; }
  Player p1 { get; set; }
  Player p2 { get; set; }

  public Game(Player P1, Player P2) {
    id = Guid.NewGuid().ToString();
    p1 = P1;
    p2 = P2;
    turn = P1.userid;
    int[,] a = {{0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0},
                {0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0},
                {0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0},
                                    {0,0,0,0,0,0,0,0,0}};
    state = a;
    actviebord = 10;
  }

  public void loop(){

  }

  void move(string player, int move) {
    if (player != turn)
      return;
    if (actviebord < 9)
      state[actviebord,move] = (player == p1.userid) ? 1 : 2;
    else
      actviebord = move;
    turn = (turn == p1.userid) ? p2.userid : p1.userid;
  }
}

class Player {
  public string userid {get; set;}
  WebSocket socket {get; set;}
  public byte[] kode {get; set;}

  public Player(string newuserid, WebSocket websocket, byte[] join){
    userid = newuserid;
    socket = websocket;
    kode = join;
  }
}


