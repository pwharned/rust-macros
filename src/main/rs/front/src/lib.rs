use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, Event};
use yew::prelude::*;
use serde_wasm_bindgen;
use macros::generate_structs_only;
use wasm_bindgen_futures::spawn_local;
use serde::{Serialize, Deserialize};
use web_sys::window;
use yew::use_effect_with;
generate_structs_only!("../server/schema.sql");



#[function_component(DataTile)]
pub fn data_tile() -> Html {
    // State to store the list of assets
    let assets = use_state(|| Vec::<assets>::new());
    let error = use_state(|| None::<String>);

    // Fetch data using `use_effect_with`
    {
        let assets = assets.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Some(window) = window() {
                    let fetch_result = window.fetch_with_str("/api/assets");
                    match wasm_bindgen_futures::JsFuture::from(fetch_result).await {
                        Ok(response) => {
                            let response: web_sys::Response = response.dyn_into().unwrap();
                            if response.ok() {
                                match wasm_bindgen_futures::JsFuture::from(response.json().unwrap()).await {
                                    Ok(json) => {
                                        match serde_wasm_bindgen::from_value::<Vec<assets>>(json) {
                                            Ok(parsed_assets) => {
                                                assets.set(parsed_assets);
                                            }
                                            Err(err) => {
                                                error.set(Some(format!("Error parsing JSON: {:?}", err)));
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        error.set(Some(format!("Error reading response: {:?}", err)));
                                    }
                                }
                            } else {
                                error.set(Some(format!(
                                    "HTTP Error: {} - {}",
                                    response.status(),
                                    response.status_text()
                                )));
                            }
                        }
                        Err(err) => {
                            error.set(Some(format!("Error fetching data: {:?}", err)));
                        }
                    }
                } else {
                    error.set(Some("Browser does not support window object.".to_string()));
                }
            });
            || ()
        });
    }

    html! {
        <div class="tile">
            <h2>{ "Assets List" }</h2>
            {
                if assets.is_empty() {
                    if let Some(ref err) = *error {
                        html! { <p style="color: red;">{ err }</p> }
                    } else {
                        html! { <p>{ "Loading..." }</p> }
                    }
                } else {
                    html! {
                        <ul>
                            { for assets.iter().map(|asset| html! {
                                <li>
                                    <cds-tile-group>
                                        <cds-radio-tile><strong>{ "ID: " }</strong>{ &asset.asset_id.clone().unwrap() }</cds-radio-tile>
                                        <cds-radio-tile><strong>{ "Description: " }</strong>{ &asset.asset_description.clone().unwrap() }</cds-radio-tile>
                                        <cds-radio-tile><strong>{ "Owner: " }</strong>{ &asset.asset_owner.clone().unwrap() }</cds-radio-tile>
                                    </cds-tile-group>
                                </li>
                            })}
                        </ul>
                    }
                }
            }
        </div>
    }
}





#[function_component(App)]
fn app() -> Html {
    let on_submit = Callback::from(move |event: Event| {
        web_sys::console::log_1(&"Submit button clicked!".into());

        // Access the Web Component dynamically
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // Example: Interact with the cds-text-input component
        if let Some(text_input) = document.get_element_by_id("text-input-1") {
            let text_input: HtmlElement = text_input.dyn_into().unwrap();
            let value = text_input.get_attribute("value").unwrap_or_default();
            web_sys::console::log_1(&format!("Text input value: {}", value).into());
        }
    });

    let on_checkbox_change = Callback::from(move |event: Event| {
        web_sys::console::log_1(&"Checkbox state changed!".into());
    });

    html! {
        <>
            <style>
                {"
                /* Add any custom styles here */
                .cds-demo {
                    margin: 1em;
                }
                "}
            </style>
                <div>
                <DataTile />
                </div>
        </>
    }
}

#[wasm_bindgen(start)]
pub fn main() {

   yew::Renderer::<App>::new().render();
}


