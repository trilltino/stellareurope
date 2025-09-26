pub mod components;
pub mod pages;
pub mod routing;
pub mod services;
pub mod utils;
pub mod hooks;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use routing::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}