use enigo::{Enigo, Key, KeyboardControllable};
use std::thread;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new();

    println!("Hello World! Let's avoid sleep for a while... ❤️ ");
    println!("To quit, use ctrl-C.");
    loop {
        // We sleep for 2 minutes
        thread::sleep(Duration::from_secs(120));
        enigo.key_up(Key::Raw(0x69));
        println!(
            "Pressed F13 at {}",
            chrono::offset::Utc::now().format("%Y-%m-%d @ %H:%M:%S")
        );
    }
}
