pub mod local;
pub mod online;
pub mod bot;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use yew::{function_component,Html,html,};
use yew_router::components::Link;
use crate::Route;
use crate::comp::modal::Modal;
use crate::model::{Model,B};
use gloo::net::websocket::Message;

#[derive(PartialEq,Clone,Serialize,Deserialize)]
pub struct GameState {
    pub mainboard:[[u8;9];10],
    pub activeboard:usize,
    pub turn:bool,
}
impl GameState {pub fn new() -> Self{Self{mainboard:[[0;9];10],activeboard:10,turn:true}}}

#[derive(Clone)]
pub struct Remote {
    pub msg:Arc<Mutex<Option<Message>>>,
}

impl PartialEq for Remote {
    fn eq(&self, other: &Self) -> bool {true}
}



#[function_component(Menu)]
pub fn menu() -> Html{

    return html!{
        <Modal>
            <Link<Route> to={Route::Online}>{ "Online" }</Link<Route>>
            <Link<Route> to={Route::Local }>{ "Local"  }</Link<Route>>
            <Link<Route> to={Route::Bot   }>{ "Bot"    }</Link<Route>>
        </Modal>
    }
}


#[derive(PartialEq,Clone)]
pub enum Oponent {
    Model(Model<B>),
    Local,
    Remote(Remote)
}
