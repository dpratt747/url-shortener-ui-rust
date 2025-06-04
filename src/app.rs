
use yew::prelude::*;
use crate::components::url_shortener::UrlShortener;
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="d-flex flex-column align-items-center justify-content-center min-vh-100">
            <UrlShortener/>
        </main>
    }
}
