use wasm_bindgen_test::{wasm_bindgen_test_configure, wasm_bindgen_test};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
extern crate webgl_thing;
use webgl_thing::vec_lib::{
    mat4::Mat4f
};

wasm_bindgen_test_configure!(run_in_browser);


// This runs a unit test in native Rust, so it can only use Rust APIs.
#[test]
fn rust_test() {
    assert_eq!(1, 1);
}


// This runs a unit test in the browser, so it can use browser APIs.
#[wasm_bindgen_test]
fn web_test() {
    assert_eq!(1, 1);
}

#[test]
fn mat4_multiply(){
    let mat1 = Mat4f::perspective(10.0f32, 1.0f32, 2.0f32, 1000f32);
    panic!();
}

// This runs a unit test in the browser, and in addition it supports asynchronous Future APIs.
#[wasm_bindgen_test(async)]
async fn async_test(){
    // Creates a JavaScript Promise which will asynchronously resolve with the value 42.
    let promise = js_sys::Promise::resolve(&JsValue::from(42));

    // Converts that Promise into a Future.
    // The unit test will wait for the Future to resolve.
    let x = JsFuture::from(promise).await.unwrap();

    assert_eq!(x, 42);
}
