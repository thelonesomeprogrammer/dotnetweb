use yew::prelude::*;
use wasm_bindgen::UnwrapThrowExt;
use burn::{ module::Module, prelude::Device, record::{ BinBytesRecorder, FullPrecisionSettings, Recorder }};
use crate::{
    comp::kryds::quartrant::Quadrant,
    func::kryds::check_win,
    model::{Model,B},
    pages::kryds::{GameState,Oponent},
};

static STATE: &[u8] = include_bytes!("../../../model.bin");

#[function_component(Krydsbole)]
pub fn krydsbole() -> Html {
    let gamestate = use_state(|| GameState::new());
    let device:Device<B> = Default::default();
    let record = BinBytesRecorder::<FullPrecisionSettings>::default()
        .load(STATE.to_vec(), &device)
        .unwrap_throw();
    let model: UseStateHandle<Oponent> = use_state(|| Oponent::Model(Model::new(&device).load_record(record)));


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
                                html!{<Quadrant gamestate={gamestate.clone()} oponent={ model.clone()} {boardid} />}
                            }).collect::<Html>()
                        }}
                    </div>
                    <div class = { classes!("line", "line-r") }></div>
                </div>
            </div>
        </div>
    }
}




