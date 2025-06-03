use wasm_bindgen::JsCast;
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let url_state = use_state(|| String::new());

    // let on_submit_url = {
    //     let url_state = url_state.clone();
    //     move |ev: SubmitEvent| {
    //         ev.prevent_default();
    //         console::log_1(&"submission pressed".into());
    //         // let value = "New URL".to_string();
    //         // url_state.set(value);
    //         web_sys::window().unwrap().alert_with_message(&format!("URL Submitted: {}", *url_state)).unwrap();
    //     }
    // };

    let on_submit_url = {
        let url_state = url_state.clone();
        Callback::from(move |ev: SubmitEvent| {
            ev.prevent_default();
            if let Some(value) = web_sys::window().unwrap()
                .document().unwrap()
                .get_element_by_id("urlInput")
                .and_then(|el| el.dyn_ref::<HtmlInputElement>().map(|input| input.value()))
            {
                url_state.set(value.clone());
                // web_sys::window().unwrap().alert_with_message(&format!("URL Submitted: {}", value.clone())).unwrap();
            }
        })
    };

    html! {
        <main class="d-flex flex-column align-items-center justify-content-center min-vh-100">

            <div>{ (*url_state).clone() }</div>

            <div>
                <form onsubmit={on_submit_url}>
                    <div class="mb-3">
                        <label for="urlInput" class="form-label fw-bold">{ "Enter a URL:" }</label>
                        <input class="form-control mb-2" type="url" id="urlInput" name="urlInput" placeholder="Enter a long URL here" style="width: 100%;" required=true/>
                        // <input class="form-control mb-2" type="url" id="urlInput" name="urlInput" placeholder="Enter a long URL here" style="width: 100%;" required=true oninput={on_input_url} />
                        <button class="btn btn-outline-success" type="submit">{ "Submit" }</button>
                    </div>
                </form>
            </div>
        </main>

    }
}
