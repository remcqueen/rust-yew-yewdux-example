use yew::{function_component, html, Html};
use yewdux::prelude::use_store;

use crate::structs::state::CounterState;

#[function_component]
pub fn Counter() -> Html {
    let (state, dispatch) = use_store::<CounterState>();
    let onclick = dispatch.reduce_mut_callback(|state| state.count += 1);

    html! {
        <>
            <p class="header">{"Count in counter: "} { state.count }</p>
            <button {onclick}>{"+1"}</button>
        </>
    }
}
