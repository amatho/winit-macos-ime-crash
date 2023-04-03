use std::{sync::Arc, thread, time::Duration};

use winit::{
    dpi::LogicalPosition,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let window = Arc::new(window);

    thread::spawn(move || {
        // Wait for event loop to initialize
        thread::sleep(Duration::from_secs(1));

        // This crashes the program on macOS
        window.set_ime_position(LogicalPosition::new(42, 42));
    });

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            control_flow.set_exit();
        }
    });
}
