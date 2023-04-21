use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, MouseEvent, KeyEvent};
use crate::log;
use crate::vec_lib::vec3::Vec3f;
use crate::webgl_utils::camera::FPSCamera;

struct InputManagerContents{
    camera: FPSCamera,
    last_time: f64,
}

pub struct InputManager{
    contents: Rc<RefCell<InputManagerContents>>
}

impl InputManager {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        let time = js_sys::Date::now() / 1000.0 % 100.0;
        let camera = FPSCamera::new(
            Vec3f::new(0.0, 0.0, -1.0),
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
            45.0,
            1280.0 / 960.0,
            0.1,
            1000.0,
        );

        let cell = RefCell::new(InputManagerContents {
            camera,
            last_time: time
        });
        let rc = Rc::new(cell);

        let input_manager = InputManager { contents: rc.clone() };

        log!("done");
        let mouse_down_closure = Closure::<dyn Fn(_)>::new(move |event: MouseEvent| {
            (*rc.borrow_mut()).mouse_down(event);
        });
        canvas.add_event_listener_with_callback("mousedown", mouse_down_closure.as_ref().unchecked_ref());
        mouse_down_closure.forget();
        input_manager
    }
}

impl InputManagerContents{
    fn mouse_down(&self, mouse_event: MouseEvent){
        log!("down");
    }

    fn mouse_up(&self, mouse_event: MouseEvent){

    }

    fn key_down(&self, key_event: KeyEvent){

    }

    fn key_up(&self, mouse_event: KeyEvent){

    }
}