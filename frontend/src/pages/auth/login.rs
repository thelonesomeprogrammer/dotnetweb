use yew::{function_component, html, Html};
use crate::comp::modal::Modal;

#[function_component( Login )]
pub fn main() -> Html {

    let form_style = "text-align: center;";
    let label_style = "font-weight: bold; display: block; margin-bottom: 5px;";
    let input_text_style = "width: 100%; padding: 10px; margin-bottom: 20px; border: 1px solid #ccc; border-radius: 5px; box-sizing: border-box;";
    let input_submit_style = "width: 100%; padding: 10px; background-color: #4CAF50; color: white; border: none; border-radius: 5px; cursor: pointer;";


    return html! {
        <Modal>
            <form style = {form_style} method="post" action="/login">
                <label style = {label_style} for="email"> {"email:"} </label> <br />
                <input style = {input_text_style} type="email" id="email" name="email" placeholder="code" value="" /> <br />
                <label style = {label_style} for="password"> {"password:"} </label> <br />
                <input style = {input_text_style} type="text" id="password" name="password" placeholder="code" value="" /> <br />
                <input style = {input_submit_style} type="submit" value="Submit" />
            </form>
        </Modal>
    };
}

