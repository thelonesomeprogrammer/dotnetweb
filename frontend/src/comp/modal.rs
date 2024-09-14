use yew::{function_component, html, Html, Properties};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}


#[function_component( Modal )]
pub fn main(props: &Props) -> Html {

    let body_style = "height:100vh; font-family: Arial, sans-serif; background-color: #f2f2f2;";
    let main_style = " padding-top: 80px; display: flex; justify-content: center;";
    let join_style = "background-color: #ffffff; padding: 20px; border-radius: 8px; box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);";


    return html! {
        <div style = {body_style}>
            <div style = {main_style}>
                <div style = {join_style}>
                    {props.children.clone()}
                </div>
            </div>
        </div>
    };
}

