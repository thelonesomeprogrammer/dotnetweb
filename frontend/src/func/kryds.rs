use burn::{
    backend::NdArray, prelude::{Backend,Tensor}, tensor::cast::ToElement
};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::UnwrapThrowExt;
use crate::{
    state::kryds::{GameState,Remote,Oponent},
    model::Model 
};
use gloo::net::websocket::Message;

impl Remote {
    pub async fn play(&mut self,mov:u8) {
        *self.msg.lock().unwrap_throw() = Some(Message::Bytes([109,58,mov].to_vec()));
    }
}

pub fn play(gamestate:GameState,action:usize,oponent:Oponent) -> GameState {
    let mut new = gamestate.clone();
    if action > 10{return new}
    new.log.push((new.turn,action.to_u8()));
    let old_active = new.activeboard;
    if new.mainboard[9][action] > 0 {
        new.activeboard = 9;
    } else {
        if old_active % 11 == 9{
            new.turn = !new.turn;
        }
        new.activeboard = action;
    }
    if old_active % 11 < 9 {
        let shape = if gamestate.turn {1} else {2};
        new.mainboard[old_active%11][action] = shape;
        if check_win(new.mainboard[old_active%11]) == shape {
            new.mainboard[9][old_active%11] = shape;
            if action == old_active%11{
                new.activeboard = 9
            }
        } else if !new.mainboard[old_active%11].contains(&0){
            new.mainboard[9][old_active%11] = 4;
        }
    }
    new.turn = !new.turn;
    if !new.turn {
        match oponent.clone() {
            Oponent::Model(model) => play(new.clone(),bot_turn(&model,new).unwrap_throw(),oponent),
            Oponent::Local => new,
            Oponent::Remote(mut remote) => {spawn_local(async move {remote.play(action as u8).await}); new.activeboard += 11;new},
        }
    }else{new}
}

pub fn make_model_state<B: Backend>(state:GameState,device: &B::Device) -> Tensor<B,2> {
    let mut new_state = [0.0;92];
    let mut index = 0;
    for i in state.mainboard.iter(){
        for &j in i.iter(){
            new_state[index] = f32::from(j);
            index += 1;
        }
    }
    new_state[index] = horyble_conv(state.activeboard);
    new_state[index+1] = if state.turn {1.0}else{2.0};

    Tensor::<B,2>::from_floats([new_state],device)
}

pub fn horyble_conv(input:usize) -> f32{
    match input {
        0 => 0.0,
        1 => 1.0,
        2 => 2.0,
        3 => 3.0,
        4 => 4.0,
        5 => 5.0,
        6 => 6.0,
        7 => 7.0,
        8 => 8.0,
        9 => 9.0,
        10 => 10.0,
        11 => 11.0,
        _ => 11.0,
    }
}

pub fn bot_turn(
    model:&Model<NdArray>,
    gamestate:GameState) -> Result<usize,burn::tensor::DataError> {
    let gamestate = gamestate.clone();
    let device = model.device();
    let input = make_model_state(gamestate.clone(),&device);
    let action = model.forward(input.clone(),&gamestate);
    let action_vec: Vec<f32> = action.into_data().to_vec()?;
    let (max_idx, _max_val) = action_vec.iter().enumerate()
        .fold((0, action_vec[0]), |(idx_max, val_max), (idx, val)| {
            if &val_max > val {(idx_max, val_max)} else {(idx, *val)}});

    Ok(max_idx)
}

pub fn check_win(input:[u8;9]) -> u8{
    let mut check:[u8;9] = [0;9];
    for i in 1..=2{
        let wins = [[i,i,i,0,0,0,0,0,0,],
                    [0,0,0,i,i,i,0,0,0,],
                    [0,0,0,0,0,0,i,i,i,],
                    [i,0,0,i,0,0,i,0,0,],
                    [0,i,0,0,i,0,0,i,0,],
                    [0,0,i,0,0,i,0,0,i,],
                    [i,0,0,0,i,0,0,0,i,],
                    [0,0,i,0,i,0,i,0,0,]];
        for &j in wins.iter() {
            for (ind,&val) in j.iter().enumerate(){
                check[ind] = input[ind] & val;
            }
            if check == j{
                return i
            }
        }
    }
    0
}


