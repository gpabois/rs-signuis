use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class="tile is-ancestor is-vertical">
           {404}
        </div>
    }
}