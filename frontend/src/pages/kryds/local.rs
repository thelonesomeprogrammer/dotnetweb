use yew::prelude::*;
use crate::{
    comp::kryds::quartrant::Quadrant,
    func::kryds::check_win,
    pages::kryds::{GameState,Oponent},
};



#[function_component(Krydsbole)]
pub fn krydsbole() -> Html {
    let gamestate = use_state(|| GameState{mainboard:[[0;9];10],activeboard:11,turn:true});
    let oponent = use_state(|| Oponent::Local);
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
                                html!{<Quadrant oponent={oponent.clone()} gamestate = {gamestate.clone()} {boardid} />}
                            }).collect::<Html>()
                        }}
                    </div>
                    <div class = { classes!("line", "line-r") }></div>
                </div>
            </div>
        </div>
    }
}
