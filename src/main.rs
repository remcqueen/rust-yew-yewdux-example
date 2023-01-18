use yew::prelude::*;
use yewdux::prelude::*;

use crate::{components::counter::Counter, structs::state::CounterState};

pub mod components;
pub mod structs;

#[function_component]
fn App() -> Html {
    let (state, _dispatch) = use_store::<CounterState>();

    html! {
        <div class="card">
        <h1 class="card__title">{"Count in main: "} {state.count}</h1>
        <Counter/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
