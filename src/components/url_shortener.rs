use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, HtmlInputElement, Request, Response, SubmitEvent};
use yew::{function_component, html, use_state, Callback, Html};

#[function_component(UrlShortener)]
pub fn url_shortener() -> Html {
    let long_url_state = use_state(|| String::new());
    let short_url_state = use_state(|| String::new());

    let url_shortener_input_field_name = "url-input";

    let on_submit_url = {
        let long_url_state = long_url_state.clone();
        let short_url_state = short_url_state.clone();
        Callback::from(move |ev: SubmitEvent| {
            ev.prevent_default();
            if let Some(long_url) = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id(url_shortener_input_field_name)
                .and_then(|element| {
                    element
                        .dyn_ref::<HtmlInputElement>()
                        .map(|input| input.value())
                })
            {
                let long_url_state = long_url_state.clone();
                let short_url_state = short_url_state.clone();
                long_url_state.set(long_url.clone());

                wasm_bindgen_futures::spawn_local(async move {
                    let shorten_endpoint = "http://localhost:8080/v1/shorten";

                    let payload = JsValue::from_str(&format!(r#"{{"longUrl": "{}"}}"#, long_url));

                    let request_init = web_sys::RequestInit::new();
                    request_init.set_method("POST");
                    request_init.set_body(&payload);

                    let headers = web_sys::Headers::new().unwrap();
                    headers.set("Content-Type", "application/json").unwrap();
                    request_init.set_headers(&headers);

                    match Request::new_with_str_and_init(&shorten_endpoint, &request_init) {
                        Ok(request) => {
                            let window = web_sys::window().unwrap();
                            match JsFuture::from(window.fetch_with_request(&request)).await {
                                Ok(resp_value) => {
                                    match resp_value.dyn_into::<Response>() {
                                        Ok(response) => {
                                            match JsFuture::from(response.json().unwrap()).await {
                                                Ok(json_value) => match json_value.as_string() {
                                                    Some(short_url) => {
                                                        short_url_state.set(short_url.clone());
                                                    }
                                                    None => {
                                                        console::log_1(&"Unable to convert response to short url string".into());
                                                    }
                                                },
                                                Err(e) => console::log_2(
                                                    &"Failed to parse json.".into(),
                                                    &e.into(),
                                                ),
                                            }
                                        }
                                        Err(e) => {
                                            console::log_2(&"Response JsValue cast to Response not successful.".into(), &e.into());
                                        }
                                    }
                                }
                                Err(e) => {
                                    console::log_2(
                                        &"Retrieving window not successful.".into(),
                                        &e.into(),
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            console::log_2(&"Request not successful.".into(), &e.into());
                        }
                    }
                });
            }
        })
    };

    html! {
        <div class="url-shortener">
            <div class="mb-3">{ (*long_url_state).clone() }</div>
            <div class="mb-3">
                if !(*short_url_state).is_empty() {
                    <a href={(*short_url_state).clone()} target="_blank" rel="noopener noreferrer">
                        { (*short_url_state).clone() }
                    </a>
                }
            </div>
            <div>
                <form onsubmit={on_submit_url}>
                    <div class="mb-3">
                        <label for={url_shortener_input_field_name} class="form-label fw-bold">{ "Enter a URL:" }</label>
                        <input class="form-control mb-2" type="url" id={url_shortener_input_field_name} name={url_shortener_input_field_name} placeholder="Enter a long URL here" style="width: 100%;" required=true/>
                        <button class="btn btn-outline-success" type="submit">{ "Submit" }</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
