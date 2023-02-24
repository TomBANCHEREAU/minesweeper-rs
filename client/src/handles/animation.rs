use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};

use crate::utils::get_window;

pub struct AnimationHandle {
    handle_id: Rc<RefCell<i32>>,
    _closure: Rc<RefCell<Option<Closure<dyn Fn()>>>>,
}

impl AnimationHandle {
    pub fn new<F: Fn() + 'static>(function: F) -> Self {
        let _closure = Rc::new(RefCell::new(None::<Closure<dyn Fn()>>));
        let closure_cloned = _closure.clone();
        let handle_id = Rc::new(RefCell::new(0));
        let handle_id_cloned = handle_id.clone();
        *_closure.borrow_mut() = Some(Closure::<dyn Fn()>::new(move || {
            function();
            *handle_id_cloned.borrow_mut() = get_window()
                .request_animation_frame(
                    closure_cloned
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unchecked_ref(),
                )
                .unwrap();
        }));
        *handle_id.borrow_mut() = get_window()
            .request_animation_frame(_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
        AnimationHandle {
            _closure,
            handle_id,
        }
    }
}

impl Drop for AnimationHandle {
    fn drop(&mut self) {
        get_window()
            .cancel_animation_frame(*self.handle_id.borrow())
            .unwrap();
    }
}
