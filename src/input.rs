use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, MouseEvent, KeyboardEvent};
use crate::log;
use crate::vec_lib::mat4::Mat4f;
use crate::vec_lib::vec3::Vec3f;
use crate::webgl_utils::camera::FPSCamera;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Keys{
    KeyW = 0,
    KeyA,
    KeyS,
    KeyD,
    KeySpace,
    KeyLShift,
}

struct InputManagerContents{
    camera: FPSCamera,
    last_time: f64,
    keys_down: [bool;6]
}

pub struct InputManager{
    contents: Rc<RefCell<InputManagerContents>>
}

const MOVE_SPEED : f32 = 1.0;
const ROT_SPEED : f32 = 0.01;

impl InputManager {
    pub fn new(canvas: &HtmlCanvasElement, window: &web_sys::Window) -> Self {
        let time = js_sys::Date::now();
        let camera = FPSCamera::new(
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(0.0, 0.0, 1.0),
            Vec3f::new(0.0, 1.0, 0.0),
            45.0,
            1280.0 / 960.0,
            0.1,
            1000.0,
        );

        let cell = RefCell::new(InputManagerContents {
            camera,
            last_time: time,
            keys_down: [false; 6]
        });
        let rc = Rc::new(cell);

        let input_manager = InputManager { contents: rc.clone() };

        let rc_closure1 = rc.clone();
        let rc_closure2 = rc.clone();
        let rc_closure3 = rc.clone();
        let rc_closure4 = rc.clone();
        let rc_closure5 = rc.clone();

        let mouse_down_closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            (*rc_closure1.borrow_mut()).mouse_down(event);
        });
        let mouse_up_closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            (*rc_closure2.borrow_mut()).mouse_up(event);
        });
        let mouse_move_closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            (*rc_closure5.borrow_mut()).mouse_move(event);
        });
        let key_down_closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
            (*rc_closure3.borrow_mut()).key_down(event);
        });
        let key_up_closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
            (*rc_closure4.borrow_mut()).key_up(event);
        });
        let poll_input_closure = Closure::<dyn FnMut()>::new(move || {
            (*rc.borrow_mut()).poll_keys();
        });

        canvas.add_event_listener_with_callback("mousedown", mouse_down_closure.as_ref().unchecked_ref());
        canvas.add_event_listener_with_callback("mouseup", mouse_up_closure.as_ref().unchecked_ref());
        canvas.add_event_listener_with_callback("mousemove", mouse_move_closure.as_ref().unchecked_ref());
        window.add_event_listener_with_callback("keydown", key_down_closure.as_ref().unchecked_ref());
        window.add_event_listener_with_callback("keyup", key_up_closure.as_ref().unchecked_ref());
        window.set_interval_with_callback_and_timeout_and_arguments_0(
        poll_input_closure.as_ref().unchecked_ref(), 4);

        mouse_down_closure.forget();
        mouse_up_closure.forget();
        mouse_move_closure.forget();
        key_down_closure.forget();
        key_up_closure.forget();
        poll_input_closure.forget();

        input_manager
    }

    pub fn proj_matrix(&self) -> Mat4f{
        self.contents.borrow_mut().camera.proj_matrix()
    }

    pub fn view_matrix(&self) -> Mat4f{
        self.contents.borrow_mut().camera.view_matrix()
    }

    pub fn position(&self) -> Vec3f{
        self.contents.borrow_mut().camera.position()
    }
}

impl InputManagerContents{
    fn mouse_down(&mut self, mouse_event: MouseEvent){
        // log!("mouse down");
    }

    fn mouse_move(&mut self, mouse_event: MouseEvent){

        let dx = mouse_event.movement_x() as f32;
        let dy = mouse_event.movement_y() as f32;

        if (mouse_event.buttons() & 0x1) == 1{
            // log!("mouse move");
            self.camera.rotate(&self.camera.up_initial(), -dx*ROT_SPEED);
            self.camera.rotate(&self.camera.right(), -dy*ROT_SPEED);
        }
    }

    fn mouse_up(&mut self, mouse_event: MouseEvent){
        // log!("mouse up");
    }

    fn key_down(&mut self, key_event: KeyboardEvent){
        // log!("key down");
        let key = key_event.key().to_lowercase();
        match key.as_str() {
            "w" =>{
                self.keys_down[Keys::KeyW as usize] = true;
            },
            "a" =>{
                self.keys_down[Keys::KeyA as usize] = true;
            },
            "s" =>{
                self.keys_down[Keys::KeyS as usize] = true;
            },
            "d" =>{
                self.keys_down[Keys::KeyD as usize] = true;
            },
            "shift" =>{
                self.keys_down[Keys::KeyLShift as usize] = true;
            }
            " " =>{
                self.keys_down[Keys::KeySpace as usize] = true;
                key_event.prevent_default();
            },
            _ =>{
                log!("Pressed key '{}'", key);
            }
        };
    }

    fn key_up(&mut self, key_event: KeyboardEvent){
        // log!("key up");
        let key = key_event.key().to_lowercase();
        match key.as_str() {
            "w" =>{
                self.keys_down[Keys::KeyW as usize] = false;
            },
            "a" =>{
                self.keys_down[Keys::KeyA as usize] = false;
            },
            "s" =>{
                self.keys_down[Keys::KeyS as usize] = false;
            },
            "d" =>{
                self.keys_down[Keys::KeyD as usize] = false;
            },
            "shift" =>{
                self.keys_down[Keys::KeyLShift as usize] = false;
            }
            " " =>{
                self.keys_down[Keys::KeySpace as usize] = false;
            },
            _ =>{
                log!("Pressed key '{}'", key);
            }
        };
    }

    fn poll_keys(&mut self){
        let time = js_sys::Date::now();
        let time_delta = time - self.last_time;
        let delta_mod = f32::min(time_delta as f32, 1.0) / 30.0;
        self.last_time = time;
        // log!("{}", delta_mod);

        if self.keys_down[Keys::KeyW as usize]{
            self.camera.translate(&self.camera.forward().scale(delta_mod * MOVE_SPEED));
        }
        if self.keys_down[Keys::KeyA as usize]{
            self.camera.translate(&self.camera.right().scale(-delta_mod * MOVE_SPEED));
        }
        if self.keys_down[Keys::KeyS as usize]{
            self.camera.translate(&self.camera.forward().scale(-delta_mod * MOVE_SPEED));
        }
        if self.keys_down[Keys::KeyD as usize]{
            self.camera.translate(&self.camera.right().scale(delta_mod * MOVE_SPEED));
        }
        if self.keys_down[Keys::KeyLShift as usize]{
            self.camera.translate(&self.camera.up().scale(-delta_mod * MOVE_SPEED));
        }
        if self.keys_down[Keys::KeySpace as usize]{
            self.camera.translate(&self.camera.up().scale(delta_mod * MOVE_SPEED));
        }
    }
}