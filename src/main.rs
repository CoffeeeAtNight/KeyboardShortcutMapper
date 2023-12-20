#![windows_subsystem = "windows"]
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::*;

fn main() {
    println!("Starting Keyboard Shortcut Mapper...\nCurrent mappings are:");
    println!("• GRAVE(BubbleTea Key) + 1 = F1");
    println!("• GRAVE(BubbleTea Key) + 2 = F2");
    println!("• GRAVE(BubbleTea Key) + 3 = F3");
    println!("• GRAVE(BubbleTea Key) + 4 = F4");
    println!("-------------------------------");

    let device_state = DeviceState::new();
    let mut last_key_cache: Option<Keycode> = Default::default();
    let mut enigo = Enigo::new();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if !keys.is_empty() {
            let pressed_key = keys.first().cloned();

            if last_key_cache == pressed_key {
                continue;
            }

            if let (Some(last_key), Some(current_key)) = (last_key_cache, pressed_key) {
                if last_key == Keycode::Grave && current_key == Keycode::Key1 {
                    enigo.key_click(Key::F1);
                }

                if last_key == Keycode::Grave && current_key == Keycode::Key2 {
                    enigo.key_click(Key::F2);
                }

                if last_key == Keycode::Grave && current_key == Keycode::Key3 {
                    enigo.key_click(Key::F3);
                }

                if last_key == Keycode::Grave && current_key == Keycode::Key4 {
                    enigo.key_click(Key::F4);
                }
            }

            last_key_cache = pressed_key;
        }
    }
}
