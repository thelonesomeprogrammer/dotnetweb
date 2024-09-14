pub mod kryds;

use web_sys::{Event, HtmlInputElement, InputEvent};
use wasm_bindgen::{JsCast, UnwrapThrowExt};

pub fn decode_input(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
}
