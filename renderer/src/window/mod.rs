use std::cell::RefCell;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

#[macro_use]
mod macros;

pub struct Window {
    window: winit::window::Window,
    event_loop: RefCell<EventLoop<()>>,
    pub width: u32,
    pub height: u32
}

impl Window {
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        let size = LogicalSize::new(
            f64::from(width), 
            f64::from(height)
        );

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(name)
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap();

        Self {
            window: window,
            event_loop: RefCell::new(event_loop),
            width,
            height
        }
    }

    pub fn handle_events<F: FnMut()>(&self, mut f: F) {
        self.event_loop
            .borrow_mut()
            .run_return(|event, _, control_flow| {
                *control_flow = ControlFlow::Poll;
                match event {
                    window_event!(WindowEvent::CloseRequested) =>
                        *control_flow = ControlFlow::Exit,

                    key_pressed!(VirtualKeyCode::Escape) =>
                        *control_flow = ControlFlow::Exit,
                        
                    Event::MainEventsCleared => f(),
                    _ => (),
                }
            });
    }

    pub fn required_extensions(&self) -> Vec<*const i8> {
        ash_window::enumerate_required_extensions(self.window.raw_display_handle())
                    .unwrap()
                    .to_vec()
    }
}

unsafe impl HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.window.raw_display_handle()
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.window.raw_window_handle()
    }
}