use yew::{use_state,Html,function_component,html,UseStateHandle};
use wasm_bindgen::UnwrapThrowExt;
use burn::{ module::Module, prelude::Device, record::{ BinBytesRecorder, FullPrecisionSettings, Recorder }};
use crate::{
    comp::kryds::Game,
    model::{Model,B},
    state::kryds::{GameState,Oponent},
};

static STATE: &[u8] = include_bytes!("../../../model.bin");

#[function_component(Bot)]
pub fn main() -> Html {
    let gamestate = use_state(|| GameState::new());
    let oponent: UseStateHandle<Oponent> = use_state(|| {
        let device:Device<B> = Default::default();
        let record = BinBytesRecorder::<FullPrecisionSettings>::default().load(STATE.to_vec(), &device).unwrap_throw();
        Oponent::Model(Box::new(Model::new(&device).load_record(record)))
    });


    return html!{<Game {gamestate} {oponent}/>}
}




