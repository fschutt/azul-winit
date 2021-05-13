use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use winit::platform::windows::WindowBuilderExtWindows;
use winapi::um::winuser::{CreateMenu, AppendMenuW};
use winapi::um::winuser::{MF_STRING, MF_SEPARATOR, MF_POPUP};
use winapi::shared::basetsd::UINT_PTR;

const IDM_FILE_NEW: u16 = 1;
const IDM_FILE_OPEN: u16 = 2;
const IDM_FILE_QUIT: u16 = 3;

fn convert_widestring(input: &str) -> Vec<u16> {
    let mut v: Vec<u16> = input.chars().filter_map(|s| {
        use std::convert::TryInto;
        (s as u32).try_into().ok()
    }).collect();
    v.push(0);
    v
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();

    let menu = unsafe { CreateMenu() };

    unsafe {
        AppendMenuW(menu, MF_STRING, IDM_FILE_NEW.into(), convert_widestring("&New").as_ptr());
        AppendMenuW(menu, MF_STRING, IDM_FILE_OPEN.into(), convert_widestring("&Open").as_ptr());
        AppendMenuW(menu, MF_SEPARATOR, 0, core::ptr::null_mut());
        AppendMenuW(menu, MF_STRING, IDM_FILE_QUIT.into(), convert_widestring("&Quit").as_ptr());
    }

    let menubar = unsafe { CreateMenu() };
    unsafe {
        AppendMenuW(menubar, MF_POPUP, menu as UINT_PTR, convert_widestring("&File").as_ptr());
    }

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .with_menu(menubar)
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    },
                    WindowEvent::Command(IDM_FILE_NEW) => {
                        println!("MENU: new file clicked!");
                    },
                    WindowEvent::Command(IDM_FILE_OPEN) => {
                        println!("MENU: open clicked!");
                    },
                    WindowEvent::Command(IDM_FILE_QUIT) => {
                        println!("MENU: quit clicked!");
                        *control_flow = ControlFlow::Exit;
                    },
                    _ => { },
                }
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
