use wasm_bindgen::UnwrapThrowExt;
use yew::{Properties,UseStateHandle,Html,html,Callback,function_component,classes};
use crate::pages::kryds::{GameState,Oponent};
use crate::func::kryds::{bot_turn,check_win};


#[derive(PartialEq, Properties)]
pub struct FeldProps {
    pub gamestate: UseStateHandle<GameState>,
    pub oponent: UseStateHandle<Oponent>,
    pub boardid:usize,
    pub feldid:usize,
}

#[function_component(Feld)]
pub fn feld(props: &FeldProps) -> Html {
    let gamestate = props.gamestate.clone();
    let boardid = props.boardid;
    let feldid = props.feldid;
    let onclick = {
        let gamestate = gamestate.clone();
        let boardid = props.boardid;
        let feldid = props.feldid;
        let active = gamestate.activeboard == boardid;
        let oponent = props.oponent.clone();

        Callback::from( move |_| {
            if gamestate.mainboard[boardid][feldid] == 0 && active {
                let mut new = (*gamestate).clone();
                if new.mainboard[9][feldid] > 0 {
                    new.activeboard = 9;
                } else {
                    new.activeboard = feldid;
                }
                let shape = if gamestate.turn {1} else {2};
                new.mainboard[boardid][feldid] = shape;
                if check_win(new.mainboard[boardid]) == shape {
                    new.mainboard[9][boardid] = shape;
                }
                new.turn = !new.turn;
                gamestate.set(new.clone());
                match (*oponent).clone() {
                    Oponent::Model(model) => {gamestate.set(bot_turn(&model,new).unwrap_throw())},
                    Oponent::Local => {},
                    Oponent::Remote(mut remote) => {gamestate.set(remote.play(new))}
                }
            }
        })
    };

    let sym = match gamestate.mainboard[boardid][feldid] {
        0 => "n",
        1 => "x",
        2 => "o",
        4 => "s",
        _ => "",};

    return html!{
        <button class={classes!("krydsbole_inner_felt")} {onclick}>
            <div class = {classes!(sym)}>
                <div class = { classes!("line", "line-l") }></div>
                <div style ="width:100%;height:100%;"></div>
                <div class = { classes!("line", "line-r") }></div>
            </div>
        </button>
    }
}



