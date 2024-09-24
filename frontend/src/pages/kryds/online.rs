use futures::StreamExt;
use yew::{function_component, html, Html, use_state, classes};
use gloo::timers::callback::Interval;
use gloo::net::websocket::{Message,futures::WebSocket};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use crate::{
    comp::kryds::quartrant::Quadrant,
    func::kryds::check_win,
    pages::kryds::{GameState,Oponent,Remote},
};
use std::sync::Arc;
use futures::lock::Mutex;
use futures::{SinkExt};


#[function_component(Krydsbole)]
pub fn krydsbole() -> Html {
    let gamestate = use_state(|| GameState::new());
    let (sink,stream) = WebSocket::open("/sock/kryds").unwrap_throw().split();
    let oponent = use_state(|| Oponent::Remote(Remote { stream:Arc::new(Mutex::new(stream)),sink:Arc::new(Mutex::new(sink)) } ) );
    let _time = use_state(|| {
        let oponent = oponent.clone();
        Interval::new(10000, move || {
            let oponent = oponent.clone();
            let oponent = (*oponent).clone();
            let oponent = match oponent{
                Oponent::Remote(o) =>{Some(o.clone())},
                _=>{None}};
            if let Some(oponent) = oponent{
                spawn_local(async move {
                    let _ = oponent.sink.lock().await.send(Message::Text(String::from("p:"))).await;
                    if let Some(Ok(msg)) = oponent.stream.lock().await.next().await{
                        match msg {
                            Message::Text(msg) =>{web_sys::console::log_1(&msg.into());},
                            Message::Bytes(msg) =>{web_sys::console::log_1(&msg.into());}, 
                        }
                    }
                });
            }
        })
    });

    let class = match check_win(gamestate.mainboard[9]){
        0 => "n",
        1 => "x",
        2 => "o",
        4 => "s",
        _ => "",};

    return html!{
        <div class ={ classes!("game-con") }>
            <div class = { classes!("krydsbole") }>
                <div class = {classes!(class)}>
                    <div class = { classes!("line", "line-l") }></div>
                    <div class = {classes!("krydsbole_outer")}>
                        {{
                            (0..=8).map( move |boardid|{
                                html!{<Quadrant gamestate={gamestate.clone()} oponent={oponent.clone()} {boardid} />}
                            }).collect::<Html>()
                        }}
                    </div>
                    <div class = { classes!("line", "line-r") }></div>
                </div>
            </div>
        </div>
    }
}
