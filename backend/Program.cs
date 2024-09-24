using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Identity.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;
using Microsoft.AspNetCore.Authentication.Cookies;
using System.Net.WebSockets;
using System.Security.Claims;


var builder = WebApplication.CreateBuilder(args);

builder.Services.AddAuthentication(CookieAuthenticationDefaults.AuthenticationScheme).AddCookie();
builder.Services.AddAuthorization();

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
        if (context.WebSockets.IsWebSocketRequest && user != null && user.Identity != null && user.Identity.IsAuthenticated)
        {
            using var webSocket = await context.WebSockets.AcceptWebSocketAsync();
            var buffer = new byte[1024];
            var receiveResult = await webSocket.ReceiveAsync(new ArraySegment<byte>(buffer), CancellationToken.None);
            var code = buffer[0..8];
            var userId = user.FindFirst(ClaimTypes.NameIdentifier)?.Value;
            if (code[0..1].ToString() == "c:" && userId != null){
              var join = code[2..8];
              var p1 = new Player(userId,webSocket,join);
              if (codes.Exists(x => x.kode == p1.kode)){
                Player? p2 = codes.Find(x => x.kode == p1.kode);
                if (p2 != null){
                  codes.Remove(p2);
                  var game = new Game(p1,p2);
                  game.loop();
                }
              } else {
                codes.Add(p1);
              }
            }
        } else {
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
}

class Game
{
  string id { get; set; }
  string turn { get; set; }
  byte[,] state { get; set; }
  int actviebord { get; set; }
  bool game_over { get; set; }
  Player p1 { get; set; }
  Player p2 { get; set; }

  public Game(Player P1, Player P2) {
    id = Guid.NewGuid().ToString();
    p1 = P1;
    p2 = P2;
    turn = P1.userid;
    byte[,] a = {{0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0},{0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0},{0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0},
                {0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0},{0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0},{0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0},
                {0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0},{0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0},{0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0},
                                    {0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0}};
    state = a;
    actviebord = 10;
  }

  public void loop(){
    var buffer = new byte[1024];
    while (!game_over) {
      p1.loop();
      p2.loop();
      if (p1.recv(buffer)){
        var cmd = buffer[0..1].ToString();
        if (cmd == "p:"){
          buffer[0] = (byte)'a';
          p1.send(buffer);
        } else if (cmd == "m:") {
          move(p1,(int)buffer[2]);
          buffer[0] = (byte)'s';
          p2.send(buffer);
        } else if (cmd == "u:") {
          buffer[0] = (byte)'b';
          Array.Copy(state, 0, buffer, 2, state.Length);
          p2.send(buffer);
        }

      }
      if (p2.recv(buffer)){
        var cmd = buffer[0..1].ToString();
        if (cmd == "p:"){
          buffer[0] = (byte)'a';
          p2.send(buffer);
        } else if (cmd == "m:") {
          move(p2,(int)buffer[2]);
          buffer[0] = (byte)'s';
          p1.send(buffer);
        } else if (cmd == "u:") {
          buffer[0] = (byte)'b';
          Array.Copy(state, 0, buffer, 2, state.Length);
          p1.send(buffer);
        }
      }
    }
  }

  void move(Player player, int move) {
    if (player.userid != turn)
      return;
    if (actviebord < 9)
      state[actviebord,move] = (byte)((player.userid == p1.userid) ? 1 : 2);
    else
      actviebord = move;
    turn = (turn == p1.userid) ? p2.userid : p1.userid;
  }
}


class Player {
  byte[] next_msg {get; set; }
  byte[] current_msg {get; set; }
  byte[] recv_msg {get; set; }
  byte[] ref_msg { get; }
  Task? sendtask { get; set; }
  Task? recvtask { get; set; }
  public string userid {get; set;}
  WebSocket socket {get; set;}
  public byte[] kode {get; set;}

  public Player(string newuserid, WebSocket websocket, byte[] join){
    userid = newuserid;
    socket = websocket;
    kode = join;
    next_msg = new byte[1024];
    recv_msg = new byte[1024];
    ref_msg = new byte[1024];
    current_msg = new byte[1024];
  }

  public bool send(byte[] buffer) {
    if (next_msg == ref_msg) {
      next_msg = buffer;
      return true;
    }
    return false;
  }

  public bool recv(byte[] buffer) {
    if (recv_msg != ref_msg) {
      buffer = recv_msg;
      recv_msg = ref_msg;
      return true;
    }
    return false;
  }

  public void loop() {
    if (sendtask != null && sendtask.IsCompleted && next_msg != ref_msg) {
      current_msg = next_msg;
      next_msg = ref_msg;
      sendtask = socket.SendAsync(new ArraySegment<byte>(current_msg), WebSocketMessageType.Binary,true, CancellationToken.None);
    }
    if (recvtask != null && recvtask.IsCompleted && recv_msg != ref_msg) {
      recvtask = socket.ReceiveAsync(new ArraySegment<byte>(recv_msg), CancellationToken.None);
    }
  }
}


