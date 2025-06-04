use wasm_bindgen::JsCast;
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let url_state = use_state(|| String::new());

    let on_submit_url = |field_name: String| -> Callback<SubmitEvent> {
        let url_state = url_state.clone();
        Callback::from(move |ev: SubmitEvent| {
            ev.prevent_default();
            if let Some(value) = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id(&field_name)
                .and_then(|element| {
                    element
                        .dyn_ref::<HtmlInputElement>()
                        .map(|input| input.value())
                })
            {
                console::log_1(&"Input successfully retrieved.".into());
                url_state.set(value.clone());

                // todo: make a shorten url request and return the shortened url

                // web_sys::window().unwrap().alert_with_message(&format!("URL Submitted: {}", value.clone())).unwrap(); for debugging
            }
        })
    };

    let url_input_field_name = "urlInput".to_string();

    html! {
        <main class="d-flex flex-column align-items-center justify-content-center min-vh-100">

            <div>{ (*url_state).clone() }</div>

            <div>
                <form onsubmit={on_submit_url(url_input_field_name.clone())}>
                    <div class="mb-3">
                        <label for={url_input_field_name.clone()} class="form-label fw-bold">{ "Enter a URL:" }</label>
                        <input class="form-control mb-2" type="url" id="urlInput" name="urlInput" placeholder="Enter a long URL here" style="width: 100%;" required=true/>
                        <button class="btn btn-outline-success" type="submit">{ "Submit" }</button>
                    </div>
                </form>
            </div>
        </main>

    }
}
