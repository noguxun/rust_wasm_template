use wasm_bindgen::prelude::*;

// Declare an JS function
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// Export a function calling a JS function
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

// Called when the wasm module is instantiated
// https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html
// Shows how to manipulate DOM in the parent html page
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global window object.    
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;    

    Ok(())
}

// Export a simple function
#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}