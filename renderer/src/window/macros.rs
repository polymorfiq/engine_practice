macro_rules! window_event {
    ($evt:pat) => {
        Event::WindowEvent {event: $evt, ..}
    }
}

macro_rules! key_pressed {
    ($key:pat) => {
        Event::WindowEvent { window_id: _, event: 
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some($key),
                        ..
                    },
                ..
            }
        }
    }
}