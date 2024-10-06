using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Identity.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;
using System.Net.WebSockets;
using System.Text;
using System.Security.Claims;
using System.Collections.Concurrent;


var builder = WebApplication.CreateBuilder(args);


var connectionString = builder.Configuration.GetConnectionString("DefaultConnection");
builder.Services.AddDbContext<DbContext>(options =>
    options.UseMySql(connectionString, ServerVersion.AutoDetect(connectionString)));

builder.Services.AddAuthentication().AddCookie("Identity.Bearer");
builder.Services.AddAuthorization();

builder.Services.AddIdentityCore<User>().AddEntityFrameworkStores<DbContext>().AddApiEndpoints();


var WebSocketOptions = new WebSocketOptions
{
  KeepAliveInterval = TimeSpan.FromMinutes(6)
};

List<Player> codes = [];
ConcurrentDictionary<string,Game> games = new ConcurrentDictionary<string,Game>(10, 10);

var app = builder.Build();

app.MapIdentityApi<User>();
app.UseStaticFiles();
app.UseRouting();
app.UseAuthentication();
app.UseAuthorization();
app.MapFallbackToFile("/index.html");
app.UseWebSockets(WebSocketOptions);
app.UseEndpoints(endpoints =>
    endpoints.MapGet("/sock/kryds", async context => {
        if (context.WebSockets.IsWebSocketRequest)
        {
            using var webSocket = await context.WebSockets.AcceptWebSocketAsync();
            var buffer = new byte[128];
            var receiveResult = await webSocket.ReceiveAsync(new ArraySegment<byte>(buffer), CancellationToken.None);
            var code = buffer[0..8];
            if (code[0] == (byte)'c' && code[1] == (byte)':'){
              var join = Encoding.UTF8.GetString(code[2..8]);
              var p1 = new Player(join);
              if (codes.Exists(x => x.kode == p1.kode)){
                Player? p2 = codes.Find(x => x.kode == p1.kode);
                if (p2 != null){
                  codes.Remove(p2);
                  var game = new Game(p2.userid,p1.userid);
                  await webSocket.SendAsync(new ArraySegment<byte>(Encoding.UTF8.GetBytes("p2")), WebSocketMessageType.Binary,true, CancellationToken.None);
                  await webSocket.SendAsync(new ArraySegment<byte>(Encoding.UTF8.GetBytes(game.id)), WebSocketMessageType.Binary,true, CancellationToken.None);
                  games[p2.userid] = game;
                  await Task.Delay(TimeSpan.FromMilliseconds(100));
                  while (true){
                    await Task.Delay(TimeSpan.FromMilliseconds(100));
                    buffer = new byte[128];

                    if (game.p2_msg != null){
                      Console.WriteLine("p2_msg:" + game.p2_msg);
                      p1.send(Encoding.UTF8.GetBytes("c:"+game.p2_msg));
                      game.p2_msg = null;
                      await p1.loop(webSocket);
                    }

                    if (p1.recv(buffer)){
                      var cmd = Encoding.UTF8.GetString(buffer[0..2]);
                      if (cmd == "p:"){
                        p1.send(Encoding.UTF8.GetBytes("a:p"));
                      } else if (cmd == "m:") {
                        var cmp = game;
                        var mov =(int)buffer[2]; 
                        game.move(p1.userid,mov);
                        games.TryUpdate(p2.userid,game,cmp);
                        p1.send(Encoding.UTF8.GetBytes("a:m"));
                      } else if (cmd == "u:") {
                        Array.Copy(Encoding.UTF8.GetBytes("b:"),0,buffer,0,2);
                        Array.Copy(game.state,0,buffer,2,90);
                        p1.send(buffer);
                      }
                    }
                    await p1.loop(webSocket);
                    if (!p1.online){
                      Game? gameer;
                      games.TryRemove(p2.userid,out gameer);
                      return;
                    }
                    if (!p2.online){
                      Game? gameer;
                      games.TryRemove(p2.userid,out gameer);
                      return;
                    }
                  }
                }
              } else {
                codes.Add(p1);
                await Task.Delay(TimeSpan.FromMilliseconds(200));
                while (true){
                  await Task.Delay(TimeSpan.FromMilliseconds(100));
                  await p1.loop(webSocket);
                  if (!p1.online){
                    codes.Remove(p1);
                    return;
                  }
                  if (games.ContainsKey(p1.userid)){
                    await webSocket.SendAsync(new ArraySegment<byte>(Encoding.UTF8.GetBytes("p1")), WebSocketMessageType.Binary,true, CancellationToken.None);
                    var game = games[p1.userid];
                    await webSocket.SendAsync(new ArraySegment<byte>(Encoding.UTF8.GetBytes(game.id)), WebSocketMessageType.Binary,true, CancellationToken.None);
                    while (true){
                      buffer = new byte[128];
                      await Task.Delay(TimeSpan.FromMilliseconds(1000));
                      if (game.p1_msg != null){
                        Console.WriteLine("p1_msg:" + game.p1_msg);
                        p1.send(Encoding.UTF8.GetBytes("c:"+game.p1_msg));
                        game.p1_msg = null;
                        await p1.loop(webSocket);
                      }
                      if (p1.recv(buffer)){
                        var cmd = Encoding.UTF8.GetString(buffer[0..2]);
                        if (cmd == "p:"){
                          p1.send(Encoding.UTF8.GetBytes("a:p"));
                        } else if (cmd == "m:") {
                          var cmp = game;
                          var mov =(int)buffer[2]; 
                          game.move(p1.userid,mov);
                          games.TryUpdate(p1.userid,game,cmp);
                          p1.send(Encoding.UTF8.GetBytes("a:m"));
                        } else if (cmd == "u:") {
                          Array.Copy(Encoding.UTF8.GetBytes("b:"),0,buffer,0,2);
                          Array.Copy(game.state,0,buffer,2,90);
                          p1.send(buffer);
                        }
                      }
                      await p1.loop(webSocket);
                      if (!p1.online){
                        return;
                      }
                    }
                  }
                }
              }
            }
        } else {
            context.Response.StatusCode = StatusCodes.Status401Unauthorized;
        }
}));

app.Run();

//var state = Encoding.UTF8.GetBytes(userId);

class User : IdentityUser{}

class DbContext : IdentityDbContext<User> {
  public DbContext(DbContextOptions<DbContext> options) : base(options){}
}

class Game
{
  public string id { get; set; }
  public string turn { get; set; }
  public byte[] state { get; set; }
  public int actviebord { get; set; }
  public bool game_over { get; set; }
  public string p1 { get; set; }
  public string p2 { get; set; }
  public string? p1_msg {get; set;}
  public string? p2_msg {get; set;}

  public Game(string P1, string P2) {
    id = Guid.NewGuid().ToString();
    p1 = P1;
    p2 = P2;
    turn = P1;
    byte[] a = {0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,
                0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,
                0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,
                                    0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0};
    state = a;
    actviebord = 10;
  }

  public void move(string player, int move) {
    Console.WriteLine("mov:"+move);
    if (player != turn)
      return;
    if (actviebord < 9)
      state[actviebord*9+move] = (byte)((player == p1) ? 1 : 2);
    else
      actviebord = move;
    turn = (turn == p1) ? p2 : p1;
    if (player == p1){
      p2_msg = move.ToString();
    } else if (player == p2){
      p1_msg = move.ToString();
    }
  }
}


class Player {
  byte[] next_msg {get; set; }
  byte[] current_msg {get; set; }
  byte[] recv_msg {get; set; }
  byte[] ref_msg { get; }
  public bool new_msg {get; set;}
  public bool new_send {get; set;}
  private readonly object _sendlock = new object();
  private readonly object _recvlock = new object();
  public string userid {get; set;}
  public string kode {get; set;}
  public bool online {get; set;}

  public Player(string join){
    online = true;
    userid = Guid.NewGuid().ToString();
    kode = join;
    next_msg = new byte[128];
    recv_msg = new byte[128];
    ref_msg = new byte[128];
    current_msg = new byte[128];
  }

  public bool send(byte[] buffer) {
    lock (_sendlock){
    if (!new_send) {
      Array.Copy(buffer,0,next_msg,0,buffer.Length);
      new_send = true;
      return true;
    }
    }
    return false;
  }

  public bool recv(byte[] buffer) {
    lock (_recvlock){
    if (new_msg) {
      Array.Copy(recv_msg,0,buffer,0,recv_msg.Length);
      Array.Copy(ref_msg,0,recv_msg,0,recv_msg.Length);
      new_msg = false;
      return true;
    }
    }
    return false;
  }

  public async Task loop(WebSocket socket) {
    switch (socket.State) {
      case WebSocketState.CloseReceived:
      case WebSocketState.Closed:
      case WebSocketState.Aborted:
      case WebSocketState.CloseSent:
      case WebSocketState.None:
        online = false;
        break;
      case WebSocketState.Connecting:
        await Task.Delay(TimeSpan.FromMilliseconds(1000));
        break;
      case WebSocketState.Open:
        break;
    }
    if (online == false){return;};

    Task? sendtask = null;
    lock (_sendlock){
    if (new_send) {
      Array.Copy(next_msg,0,current_msg,0,recv_msg.Length);
      Array.Copy(ref_msg,0,next_msg,0,recv_msg.Length);
      sendtask = Task.Run(() => socket.SendAsync(new ArraySegment<byte>(current_msg), WebSocketMessageType.Binary,true, CancellationToken.None));
    }
    }
    if (!new_msg) {
      await socket.ReceiveAsync(new ArraySegment<byte>(recv_msg), CancellationToken.None);
      new_msg = true;
    }
    if (sendtask != null){
      await sendtask;
      lock (_sendlock){
        new_send = false;
      }
    }
  }
}


