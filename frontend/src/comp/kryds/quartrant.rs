use wasm_bindgen::UnwrapThrowExt;
use yew::{Properties,UseStateHandle,Html,html,Callback,function_component,classes};
use crate::pages::kryds::{GameState,Oponent};
use crate::func::kryds::bot_turn;
use crate::comp::kryds::feld::Feld;



#[derive(PartialEq, Properties)]
pub struct QuadrantProps {
    pub gamestate: UseStateHandle<GameState>,
    pub oponent: UseStateHandle<Oponent>,
    pub boardid:usize,
}


#[function_component(Quadrant)]
pub fn quadrant(props: &QuadrantProps) -> Html {
    let boardid = props.boardid;
    let gamestate = props.gamestate.clone();
    let oponent = props.oponent.clone();
    let active = gamestate.activeboard == boardid || {gamestate.activeboard < 15 && gamestate.activeboard > 8};
    let mut class:[&str;2] = ["e","e"];


    let onclick = {
        let gamestate = gamestate.clone();
        let oponent = oponent.clone();
        Callback::from( move |_| {
            if gamestate.activeboard > 8 {
                let mut new = (*gamestate).clone();
                if new.activeboard == 10{
                    new.turn = !new.turn;
                }
                new.activeboard = boardid;
                gamestate.set(new.clone());

                if !new.turn{
                    match (*oponent).clone() {
                        Oponent::Model(model) => {gamestate.set(bot_turn(&model,new).unwrap_throw())},
                        Oponent::Local => {}
                        Oponent::Remote(mut remote) => {gamestate.set(remote.play(new))}
                    }
                }   
            }
        })
    };

    class[0] = match gamestate.mainboard[9][boardid] {
        0 => "n",
        1 => "x",
        2 => "o",
        4 => "s",
        _ => "",};

    class[1] = if active {"t"} else {"f"};

    return html!{
        <div  {onclick} class = { classes!("krydsbole_outer_felt") }>
            <div class = {classes!(class)}>
                <div class = { classes!("line", "line-l") }></div>
                <div class = {classes!("krydsbole_inner")}>
                    {{
                        (0..=8).map( move |feldid| {
                            html!{
                                <Feld gamestate = {gamestate.clone()} {feldid} oponent={oponent.clone()} {boardid} />
                            }
                        }).collect::<Html>()
                    }}
                </div>
                <div class = { classes!("line", "line-r") }></div>
            </div>
        </div>
    }
}