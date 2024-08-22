
use yew::prelude::*;

#[function_component(Krydsbole)]
pub fn krydsbole() -> Html{


    return html!{
        <div class = { classes!("krydsbole") }>
            <div class = { classes!("krydsbole_outer") }>
                {{
                     (1..=3).map(|i|{
                         (1..=3).map( move |j| html!{
                            <div class = { classes!(format!{"bt{i}{j} krydsbole_outer_felt"}) }>
                                <div class = { classes!(format!{"bt{i}{j} krydsbole_inner"}) }>
                                    {
                                        (1..=3).map(|i|{
                                            (1..=3).map( move |j| html!{
                                                <div class = { classes!(format!{"bb{i}{j} krydsbole_inner_felt"}) }></div>
                                            }).collect::<Html>()
                                        }).collect::<Html>()
                                    }
                                </div>
                            </div>
                         }).collect::<Html>()
                     }).collect::<Html>() 
                 }}
            </div>
        </div>
    }
}


