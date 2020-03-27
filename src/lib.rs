use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo::{events::EventListener};
mod crypt;

// run on load
#[wasm_bindgen]
pub fn setup() {
    console_error_panic_hook::set_once();

    // setup dom
    let window: web_sys::Window = web_sys::window().expect("");    
    let document: web_sys::Document = window.document().expect("");
    document.get_element_by_id("ver").unwrap().set_inner_html(&format!("v{}", env!("CARGO_PKG_VERSION"))[..]);

    // attach elements
    let message_in: web_sys::HtmlTextAreaElement = document.get_element_by_id("message_io").unwrap().dyn_into().unwrap();
    let message_out = message_in.clone();
    let password_enc: web_sys::HtmlInputElement = document.get_element_by_id("pass_input").unwrap().dyn_into().unwrap();
    let password_dec = password_enc.clone();
    let enc_button: web_sys::Element = document.get_element_by_id("enc_button").unwrap();
    let dec_button: web_sys::Element = document.get_element_by_id("dec_button").unwrap();

    // attach button listeners
    EventListener::new(&enc_button, "click", move |_event| {
        message_in.set_value(&crypt::cipher(
            message_in.value(),
            password_enc.clone().value(),
        false)[..]);
    }).forget();
    EventListener::new(&dec_button, "click", move |_event| {
         message_out.set_value(&crypt::cipher(
            message_out.value(),
            password_dec.clone().value(),
        true)[..]);
    }).forget();
}
