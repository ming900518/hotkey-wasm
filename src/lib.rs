use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    kb_event();
}

pub fn kb_event() {
    let window = web_sys::window().expect("Global window does not exist.");
    let document = window.document().expect("Document does not exist.");

    let nav_bar = document
        .get_element_by_id("main-nav-bar")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let content = document
        .get_element_by_id("main-content")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let footer = document
        .get_element_by_id("footer")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let body = document.body().unwrap();

    let on_keydown = EventListener::new(&body, "keydown", move |event| {
        let event = event.clone().dyn_into::<web_sys::KeyboardEvent>().unwrap();
        let alt_key = &event.alt_key();
        let code: &String = &event.code();
        match code.to_string().as_str() {
            "KeyU" => {
                if *alt_key {
                    nav_bar.focus().unwrap()
                }
            }
            "KeyC" => {
                if *alt_key {
                    content.focus().unwrap()
                }
            }
            "KeyZ" => {
                if *alt_key {
                    footer.focus().unwrap()
                }
            }
            "Enter" => {
                let focused_element = document.active_element().unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
                focused_element.click();
            }
            "NumpadEnter" => {
                let focused_element = document.active_element().unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
                focused_element.click();
            }
            _ => (),
        }
    });
    on_keydown.forget();
}
