pub mod local;
pub mod online;
pub mod bot;
use serde::{Deserialize, Serialize};
use yew::{function_component,Html,html,};
use yew_router::components::Link;
use crate::Route;
use crate::comp::modal::Modal;
use crate::model::{Model,B};
use gloo::net::websocket::{Message,futures::WebSocket};
use futures::stream::{SplitSink,SplitStream};
use std::sync::Arc;
use futures::lock::Mutex;

#[derive(PartialEq,Clone,Serialize,Deserialize)]
pub struct GameState {
    pub mainboard:[[u8;9];10],
    pub activeboard:usize,
    pub turn:bool,
}
impl GameState {pub fn new() -> Self{Self{mainboard:[[0;9];10],activeboard:10,turn:true}}}

#[derive(Clone)]
pub struct Remote {
    pub stream:Arc<Mutex<SplitStream<WebSocket>>>,
    pub sink:Arc<Mutex<SplitSink<WebSocket,Message>>>,
}

impl PartialEq for Remote{
    fn eq(&self, _rhs:&Remote)->bool{true}
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
