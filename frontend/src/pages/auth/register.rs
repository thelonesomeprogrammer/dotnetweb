use serde::Serialize;
use yew::{function_component, html, Html,Callback};
use wasm_bindgen::UnwrapThrowExt;
use crate::comp::modal::Modal;
use gloo::net::http::Request;
use web_sys::{window,HtmlInputElement};
use wasm_bindgen::JsCast;

#[derive(Serialize)]
struct Regi{
    username:String,
    password:String,
    email:String,
}


#[function_component( Register )]
pub fn main() -> Html {
    let onclick = {Callback::from(move |_| {
        let form = window().unwrap_throw().document().unwrap_throw();
        let email = form.get_element_by_id("email").unwrap_throw().dyn_into::<HtmlInputElement>().unwrap_throw().value();
        let password = form.get_element_by_id("password").unwrap_throw().dyn_into::<HtmlInputElement>().unwrap_throw().value();
        let username = form.get_element_by_id("username").unwrap_throw().dyn_into::<HtmlInputElement>().unwrap_throw().value();
        wasm_bindgen_futures::spawn_local(async move {
                    Request::post("/register")
                        .json(&Regi{username,password,email})
                        .unwrap_throw()
                        .send()
                        .await
                        .unwrap();
                });
    })};

    let form_style = "text-align: center;";
    let label_style = "font-weight: bold; display: block; margin-bottom: 5px;";
    let input_text_style = "width: 100%; padding: 10px; margin-bottom: 20px; border: 1px solid #ccc; border-radius: 5px; box-sizing: border-box;";
    let input_submit_style = "width: 100%; padding: 10px; background-color: #4CAF50; color: white; border: none; border-radius: 5px; cursor: pointer;";



        return html! {
            <Modal>
                <from style = {form_style}>
                    <label style = {label_style} for="username"> {"Nickname:"} </label> <br />
                    <input style = {input_text_style} type="text" id="username" name="username" placeholder="nickname" value="" /> <br />
                    <label style = {label_style} for="email"> {"email:"} </label> <br />
                    <input style = {input_text_style} type="email" id="email" name="email" placeholder="code" value="" /> <br />
                    <label style = {label_style} for="password"> {"password:"} </label> <br />
                    <input style = {input_text_style} type="text" id="password" name="password" placeholder="code" value="" /> <br />
                    <button style = {input_submit_style} type="submit" {onclick}> {"Submit"} </button>
                </from>
            </Modal>
            
    };
}

