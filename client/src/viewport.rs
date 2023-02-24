use std::sync::{Arc, Mutex};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console::log_1, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, UiEvent};

use crate::{
    handles::{animation::AnimationHandle, click::ClickHandle, resize::ResizeHandle},
    image::{ImageManager, Sprite},
    utils::get_window,
};

pub struct ViewportOptions {
    pub canvas_id: String,
}

pub struct Viewport {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    image_manager: ImageManager,
    resize_handle: Option<ResizeHandle>,
    animation_handle: Option<AnimationHandle>,
    click_handle: Option<ClickHandle>,
}

impl Viewport {
    pub fn new(options: ViewportOptions) -> Result<(), String> {
        let canvas = get_canvas(&options.canvas_id)?;
        let context = get_context(&canvas)?;
        let mutexed_viewport = Arc::new(Mutex::new(Viewport {
            canvas,
            context,
            resize_handle: None,
            image_manager: ImageManager::new(),
            animation_handle: None,
            click_handle: None,
        }));
        let mut viewport = mutexed_viewport.lock().unwrap();

        let cloned_viewport = mutexed_viewport.clone();
        viewport.on_resize();
        viewport.resize_handle = Some(ResizeHandle::new(move || {
            cloned_viewport.lock().unwrap().on_resize();
        }));
        let cloned_viewport = mutexed_viewport.clone();
        viewport.animation_handle = Some(AnimationHandle::new(move || {
            cloned_viewport.lock().unwrap().on_animation_frame();
        }));
        let cloned_viewport = mutexed_viewport.clone();
        viewport.click_handle = Some(ClickHandle::new(
            get_canvas(&options.canvas_id)?,
            move |event: MouseEvent| {
                cloned_viewport.lock().unwrap().on_click(event);
            },
        ));
        Ok(())
    }
    fn on_click(&mut self, event: MouseEvent) {
        log_1(&JsValue::from_str("on_click"))
    }
    fn on_resize(&mut self) {
        self.canvas.set_width(self.canvas.client_width() as u32);
        self.canvas.set_height(self.canvas.client_height() as u32);
        log_1(&JsValue::from_str("on_resize"))
    }
    fn on_animation_frame(&mut self) {
        self.image_manager
            .draw_sprite(&self.context, Sprite::One, 0., 0., 16., 16.);
        log_1(&JsValue::from_str("RequestAnimationFrame"))
    }
}

fn get_canvas(canvas_id: &String) -> Result<HtmlCanvasElement, String> {
    let window = get_window();
    let document = window.document().ok_or("Could not get Document")?;
    document
        .get_element_by_id(&canvas_id)
        .ok_or(format!("Could not get element with id: \"{}\"", canvas_id))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .ok()
        .ok_or(format!(
            "Element with id: \"{}\" is not a canvas",
            canvas_id
        ))
}

fn get_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, String> {
    canvas
        .get_context("2d")
        .ok()
        .ok_or("Could not get the 2d context")?
        .ok_or("Could not get the 2d context")?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .ok()
        .ok_or("Could not get the 2d context".to_string())
}
