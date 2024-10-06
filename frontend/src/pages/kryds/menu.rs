
use yew::{function_component,Html,html,};
use yew_router::components::Link;
use crate::Route;
use crate::comp::modal::Modal;



#[function_component(Menu)]
pub fn menu() -> Html{

    return html!{
        <Modal>
            <div class="pick-oppnent-con">
                <ul class="pick-oppnent-ul">
                    <li class="pick-oppnent-li"><Link<Route> to={Route::Online}>{ "Online" }</Link<Route>></li>
                    <li class="pick-oppnent-li"><Link<Route> to={Route::Local }>{ "Local"  }</Link<Route>></li>
                    <li class="pick-oppnent-li"><Link<Route> to={Route::Bot   }>{ "Bot"    }</Link<Route>></li>
                </ul>
            </div>
        </Modal>
    }
}
