use std::clone;

use wasm_bindgen::{JsCast,UnwrapThrowExt};
use web_sys::{Event,HtmlElement};
use yew::prelude::*;

#[derive(PartialEq)]
enum Feldstate {
    None,
    X,
    O,
}

#[derive(PartialEq,Clone)]
struct GameState {
    mainboard:[[u16;10];2],
    turn:bool,
}

#[derive(PartialEq, Properties)]
struct FeldProps {
    gamestate: UseStateHandle<GameState>,
    boardid:usize,
    feldid:u8,
}

#[function_component(Krydsbole)]
pub fn krydsbole() -> Html{
    let gamestate = use_state(|| GameState{mainboard:[[0;10];2],turn:true});

    return html!{
        <div class = { classes!("krydsbole") }>
            <div class = { classes!("krydsbole_outer") }>
                {{
                     if check_win((*gamestate).mainboard[0][9]){
                         html!{"X"}
                     }else if check_win((*gamestate).mainboard[1][9]){
                         html!{
                            <div class = { classes!("krydsbole_outer_felt") }>
                                <div style={"
                                 height: 90%; 
                                 aspect-ratio:1;
                                 border-radius: 50%;
                                 background: radial-gradient(closest-side, white 79%, transparent 80% 100%), conic-gradient(black 100%, white 0);"}>
                                </div>
                            </div>
                         }
                     } else{
                         (0..=2).map(|x|{
                             let gamestate = gamestate.clone();
                             (0..=2).map( move |y|{
                                 let boardid = x*3+y;
                                 if (*gamestate).mainboard[0][9] & (1 << boardid) == 1 << boardid {
                                     html!{"X"}
                                 } else if (*gamestate).mainboard[1][9] & (1 << boardid) == 1 << boardid {
                                     html!{
                                         <div class = { classes!("krydsbole_outer_felt") }>
                                            <div style={"height: 90%; aspect-ratio:1; border-radius: 50%; background: radial-gradient(closest-side, white 79%, transparent 80% 100%), conic-gradient(black 100%, white 0);"}>
                                            </div>
                                         </div>
                                     }
                                 } else {
                                     html!{
                                         <div class = { classes!("krydsbole_outer_felt") }>
                                             <div class = { classes!("krydsbole_inner") }>
                                             {{
                                                  (0..=2).map(|x|{
                                                      let gamestate = gamestate.clone();
                                                      (0..=2).map( move |y| {
                                                          let feldid = x*3+y;
                                                          html!{
                                                              <Feld gamestate = {gamestate.clone()} {feldid} {boardid} />
                                                          }
                                                      }).collect::<Html>()
                                                  }).collect::<Html>()
                                              }}
                                         </div>
                                             </div>
                                     }
                                 }}).collect::<Html>()
                         }).collect::<Html>() 
                     }
                 }}
            </div>
        </div>
    }
}


#[function_component(Feld)]
fn feld(props: &FeldProps) -> Html {
    let gamestate = props.gamestate.clone();
    let class = classes!("krydsbole_inner_felt");
    let clicked = use_state(|| Feldstate::None);
    let onclick = {
        let clicked = clicked.clone(); 
        let gamestate = gamestate.clone();
        let boardid = props.boardid;
        let feldid = props.feldid;

        Callback::from( move |_| {
            if (*clicked) == Feldstate::None{
                clicked.set(if (*gamestate).turn {
                    gamestate.set({
                        let mut new = (*gamestate).clone();
                        new.turn=false;
                        new.mainboard[0][boardid] += 1 << feldid;
                        if check_win(new.mainboard[0][boardid]) {
                            new.mainboard[0][9] += 1 << boardid;
                        }
                        new});
                    Feldstate::X
                }else{
                    gamestate.set({
                        let mut new = (*gamestate).clone();
                        new.turn=true;
                        new.mainboard[1][boardid] += 1 << feldid;
                        if check_win(new.mainboard[1][boardid]) {
                            new.mainboard[1][9] += 1 << boardid;
                        }
                        new});
                    Feldstate::O
                });
            }
        })
    };

    return html!{
        <button {class} {onclick}>{match (*clicked) {
            Feldstate::None => {" "},
            Feldstate::X    => {"X"},
            Feldstate::O    => {"O"},
        }}</button>
    }
}

fn check_win(input:u16) -> bool{
    let win_masks = [
        0b111000000, // first row
        0b000111000, // second row
        0b000000111, // third row
        0b100100100, // first column
        0b010010010, // second column
        0b001001001, // third column
        0b100010001, // main diagonal
        0b001010100, // anti-diagonal
    ];
    for &i in win_masks.iter(){
        if input & i == i {
            return true;
        }
    }
    false
}
