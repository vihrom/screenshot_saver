use chrono::Local;
use rdev::{grab, Event, EventType, Key};
use screenshots::Screen;
use std::env;
use std::fs;

// Default directory name for saving screenshots
const TARGET_DIR: &str = "screens";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let screens_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    let mut path = env::current_dir()?;

    path.push(&screens_dir);
    fs::create_dir_all(path).unwrap_or_else(|e| {
        eprintln!("Failed to create directory: {}", e);
        std::process::exit(1);
    });

    if let Err(error) = grab(move |e| callback(e, &screens_dir)) {
        println!("Error: {error:?}");
    }

    Ok(())
}

// Callback function triggered on key events
fn callback(event: Event, screens_dir: &String) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::PrintScreen) => {
            make_screen(screens_dir);
            None
        }
        _ => Some(event),
    }
}

// Function to capture and save screenshots
fn make_screen(screens_dir: &String) {
    let screens = Screen::all().unwrap();

    for screen in screens {
        let image = screen.capture().unwrap();

        let now = Local::now();

        image
            .save(format!(
                "{}/{}.png",
                screens_dir,
                now.format("%d-%m-%Y_%H-%M-%S-%f")
            ))
            .unwrap();
    }
}
