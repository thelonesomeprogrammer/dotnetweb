use yew::prelude::{html,function_component,Html,use_state};
use crate::{comp::kryds::Game,state::kryds::{GameState,Oponent}};



#[function_component(Local)]
pub fn main() -> Html {
    let gamestate = use_state(|| GameState::new());
    let oponent = use_state(|| Oponent::Local);

    return html!{<Game {gamestate} {oponent}/>}
}
