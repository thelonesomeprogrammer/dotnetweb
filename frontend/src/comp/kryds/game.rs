use yew::{Properties,UseStateHandle,Html,html,function_component,classes};
use crate::state::kryds::{GameState,Oponent};
use crate::comp::kryds::{Quadrant,Rules,Log};
use crate::func::kryds::check_win;



#[derive(PartialEq, Properties)]
pub struct GameProps {
    pub gamestate: UseStateHandle<GameState>,
    pub oponent: UseStateHandle<Oponent>,
}


#[function_component(Game)]
pub fn main(props: &GameProps) -> Html {
    let gamestate = props.gamestate.clone();
    let oponent = props.oponent.clone();


    let class = match check_win(gamestate.mainboard[9]){
        0 => "n",
        1 => "x",
        2 => "o",
        4 => "s",
        _ => "",};

    return html!{
        <div class = { classes!("game-con") }>
            <Rules />
            <div class = { classes!("krydsbole") }>
                <div class = {classes!(class)}>
                    <div class = { classes!("line", "line-l") }></div>
                    <div class = {classes!("krydsbole_outer")}>
                        {{
                             let gamestate = gamestate.clone();
                            (0..=8).map( move |boardid|{
                                html!{<Quadrant gamestate={gamestate.clone()} oponent={oponent.clone()} {boardid} />}
                            }).collect::<Html>()
                        }}
                    </div>
                    <div class = { classes!("line", "line-r") }></div>
                </div>
            </div>
            <Log turn = {gamestate.turn} log = {gamestate.log.clone()}/>
        </div>
    }
}

