macro_rules! window_event {
    ($evt:pat) => {
        Event::WindowEvent {event: $evt, ..}
    }
}

macro_rules! key_pressed {
    ($key:pat) => {
        winit::event::Event::WindowEvent { window_id: _, event: 
            winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state: winit::event::ElementState::Pressed,
                        virtual_keycode: Some($key),
                        ..
                    },
                ..
            }
        }
    }
}

macro_rules! key_released {
    ($key:pat) => {
        winit::event::Event::WindowEvent { window_id: _, event: 
            winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state: winit::event::ElementState::Released,
                        virtual_keycode: Some($key),
                        ..
                    },
                ..
            }
        }
    }
}