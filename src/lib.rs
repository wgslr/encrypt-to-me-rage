mod utils;

use age::keys::RecipientKey;
use age::Format;
use std::fmt::Debug;
use std::io::Write;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn encrypt_to_pubkey(plaintext: &str, pubkey: &str) -> Result<String, JsValue> {
    utils::set_panic_hook();

    let key: RecipientKey = pubkey
        .parse()
        .or_else(|e| Err(JsValue::from_str(&format!("{:?}", e))))?;
    //
    let encryptor = age::Encryptor::with_recipients(vec![key]);

    let mut encrypted = vec![];
    let mut output = encryptor
        .wrap_output(&mut encrypted, Format::AsciiArmor)
        .map_err(err_to_js)?;

    output.write_all(plaintext.as_bytes()).map_err(err_to_js)?;
    output.finish().map_err(err_to_js)?;

    String::from_utf8(encrypted).or_else(|e| Err(err_to_js(e)))
}

fn err_to_js<T: Debug>(e: T) -> JsValue {
    JsValue::from_str(&format!("{:?}", e))
}
