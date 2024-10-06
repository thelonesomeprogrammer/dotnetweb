use yew::{use_state,Html,html,Callback,function_component,Properties};


#[derive(PartialEq, Properties)]
pub struct GameLogProps {
    pub turn:bool,
    pub log:Vec<(bool,u8)>,
}


#[function_component(Log)]
pub fn main(props: &GameLogProps ) -> Html {
    let turn = if props.turn{"x"}else{"o"};
    let log = props.log.clone();
    let open = use_state(|| false);
    let onclick = {
        let open = open.clone();
        Callback::from( move |_| {
            open.set(!(*open))
        })
    };

    return html!{
        <div class="ultimate-tic-tac-toe-log-con">
            <button {onclick}><h2>{"currunt player:"}{turn}</h2></button>
            <div class="ultimate-tic-tac-toe-log" style={if *open{""}else{"opacity:0;"}}>                
                <ul>
                    {{
                         log.iter().map( move |log| { html!{
                             <li>{{if log.0{"x"}else{"o"}}}{" : "}{log.1}</li>
                         }}).collect::<Html>()
                    }}
                </ul>
            </div>

        </div>
    }
}

