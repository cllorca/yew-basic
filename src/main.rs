use yew::prelude::*;
use capped_input_component::CappedInputComponent;

mod capped_input_component;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <div class="container">
            <h1>{"Basic yew web app"}</h1>
            <div>
                <CappedInputComponent/>
            </div>
        </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("App is starting");
    yew::Renderer::<App>::new().render();
}