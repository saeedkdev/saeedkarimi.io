use yew::prelude::*;
mod components;

use components::header::Header;
use components::intro::Intro;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Header />
            <div class="mx-auto w-7/12 my-5 font-open-sans">
                <Intro />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
