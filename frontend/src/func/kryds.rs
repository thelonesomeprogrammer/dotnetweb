use burn::{
    prelude::{Backend,Tensor,},
    backend::NdArray,
};
use crate::model::Model;
use crate::pages::kryds::{GameState,Remote};

impl Remote {
    pub fn play(&mut self, gamestate:GameState) -> GameState {
        let mut gamestate = gamestate.clone();
        gamestate.activeboard = 11;
        gamestate
    }
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
    gamestate:GameState) -> Result<GameState,burn::tensor::DataError> {
    let gamestate = gamestate.clone();
    let device = model.device();
    let input = make_model_state(gamestate.clone(),&device);
    let action = model.forward(input.clone(),&gamestate);
    let action_vec: Vec<f32> = action.into_data().to_vec()?;
    let (max_idx, _max_val) = action_vec.iter().enumerate()
        .fold((0, action_vec[0]), |(idx_max, val_max), (idx, val)| {
            if &val_max > val {(idx_max, val_max)} else {(idx, *val)}});

    let mut new = gamestate.clone();
    if gamestate.activeboard > 8 {
        if gamestate.activeboard == 10{
            new.turn = true;
        }
        new.activeboard = max_idx;
    }
    let boardid = gamestate.activeboard;
    let feldid = max_idx;
    if boardid > 9{return Ok(new)}
    if gamestate.mainboard[boardid][feldid] == 0{
        if new.mainboard[9][feldid] > 0 {
            new.activeboard = 9;
        } else {
            new.activeboard = feldid;
        }
        let shape = if gamestate.turn {1} else {2};
        new.mainboard[boardid][feldid] = shape;
        if check_win(new.mainboard[0]) == shape {
            new.mainboard[9][boardid] = shape;
        }
        new.turn = true;
    }
    Ok(new)
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


