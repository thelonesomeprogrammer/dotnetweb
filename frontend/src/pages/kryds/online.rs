use yew::{function_component, html, Html, use_state, classes};
use gloo::timers::callback::Interval;
use gloo::net::websocket::{Message,futures::WebSocket};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use crate::{
    comp::kryds::quartrant::Quadrant,
    func::kryds::check_win,
    pages::kryds::GameState,
};


#[function_component(Krydsbole)]
pub fn krydsbole() -> Html {
    let gamestate = use_state(|| GameState{mainboard:[[0;9];10],activeboard:11,turn:true});
    let socket = use_state(|| WebSocket::open("/sock/kryds").unwrap_throw());
    let _time = use_state(|| {
        let gamestate = gamestate.clone();
        Interval::new(10000, move || {
            let socket = socket.clone();
            spawn_local(async move {

            });
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
                                html!{<Quadrant gamestate={gamestate.clone()} model={None} {boardid} />}
                            }).collect::<Html>()
                        }}
                    </div>
                    <div class = { classes!("line", "line-r") }></div>
                </div>
            </div>
        </div>
    }
}
