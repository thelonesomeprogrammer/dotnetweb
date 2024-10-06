use std::sync::Mutex;
use std::rc::Rc;
use futures::StreamExt;
use yew::{function_component, html, use_effect_with, use_state, Html};
use gloo::timers::{future::TimeoutFuture,callback::Interval};
use gloo::net::websocket::{Message,futures::WebSocket};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use crate::func::kryds::play;
use crate::{
    comp::kryds::Game,
    state::kryds::{GameState,Oponent,Remote},
};
use futures::SinkExt;


#[function_component(Online)]
pub fn main() -> Html {
    let gamestate = use_state(|| GameState::new());
    let gamestate1 = gamestate.clone();
    let suck = use_state(|| {let (sink,stream) = WebSocket::open("/sock/kryds").unwrap_throw().split();(Rc::new(futures::lock::Mutex::new(sink)),Rc::new(futures::lock::Mutex::new(stream)))});
    let oponent = use_state(|| Oponent::Remote(Remote { msg: Rc::new(Mutex::new(Some(Message::Text(String::from("c:iamacoolkid")))))} ) );
    let oponent1 = oponent.clone();
    let incomming = use_state(|| (1,50) );
    use_effect_with(incomming.clone(), move |inc| {gamestate1.set(play((*gamestate1).clone(), inc.1, (*oponent1).clone()));}); 
    let _timesink = use_state(|| {
            let oponent = oponent.clone();
            let suck = suck.clone();
            Interval::new(100, move || {
                let oponent = oponent.clone();
                let suck = suck.clone();
                if let Oponent::Remote(opp) = (*oponent).clone() {
                    let message = opp.msg.lock().unwrap_throw().clone();
                    if let Some(msg) = message {
                        spawn_local(async move {let _ = suck.0.lock().await.send(msg).await;});
                        *opp.msg.lock().unwrap_throw() = None;
                    }
                }
            })
    });
    let _int = use_state(|| {
        let oponent = oponent.clone();
        Interval::new(10000, move || {
            let oponent = oponent.clone(); 
            if let Oponent::Remote(opp) = (*oponent).clone() {
                if opp.msg.lock().unwrap_throw().is_none(){
                        *opp.msg.lock().unwrap_throw() = Some(Message::Text(String::from("p:")));
                }
            }
        })
    });
    let _time = use_state(|| {
        let gamestate = gamestate.clone();
        let incomming = incomming.clone();
        spawn_local(async move {
            loop{
                let _out = TimeoutFuture::new(100).await;
                if let Some(Ok(msg)) = suck.1.lock().await.next().await{
                    match msg {
                        Message::Text(msg) =>{web_sys::console::log_1(&msg.into());},
                        Message::Bytes(msg) =>{
                            match msg[0..2]{
                                [97,58] => {} // a:
                                [112,49] => {} // p1
                                [112,50] => { // p2
                                    let mut new = (*gamestate).clone();
                                    new.turn = false;
                                    new.activeboard = 21;
                                    gamestate.set(new);
                                }
                                [0x63,0x3a] => { // c:
                                    web_sys::console::log_1(&(msg[2]-48).into());
                                    incomming.set((chrono::Utc::now().timestamp_millis(),(msg[2]-48) as usize));
                                }
                                _ => {
                                    web_sys::console::log_1(&msg[0..2][0].into());
                                    web_sys::console::log_1(&msg[0..2][1].into());
                                }
                            }
                        }, 
                    }
                }
            }});
        });


    return html!{<Game gamestate = {gamestate.clone()} oponent={oponent.clone()}/>}
}
