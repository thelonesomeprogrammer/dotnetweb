use yew::{Properties,UseStateHandle,Html,html,Callback,function_component,classes};
use crate::state::kryds::{GameState,Oponent};
use crate::func::kryds::play;


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
                gamestate.set(play((*gamestate).clone(),feldid,(*oponent).clone()))
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



