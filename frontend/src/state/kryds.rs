use std::sync::Mutex;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::model::{Model,B};
use gloo::net::websocket::Message;


#[derive(PartialEq,Clone,Serialize,Deserialize)]
pub struct GameState {
    pub mainboard:[[u8;9];10],
    pub activeboard:usize,
    pub turn:bool,
    pub log:Vec<(bool,u8)>,
}
impl GameState {pub fn new() -> Self{Self{mainboard:[[0;9];10],activeboard:10,turn:true,log:vec![]}}}

#[derive(Clone)]
pub struct Remote {
    pub msg:Rc<Mutex<Option<Message>>>,
}

impl PartialEq for Remote { fn eq(&self, _other: &Self) -> bool {true}}


#[derive(PartialEq,Clone)]
pub enum Oponent {
    Model(Box<Model<B>>),
    Local,
    Remote(Remote)
}
