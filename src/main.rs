use std::env;
use std::fs;
use std::path::Path;
use swayipc::{Connection, Event, EventType, Fallible};

fn main() -> Fallible<()> {

    let lang_filename = match env::var("KLAVARO_FILE") {
        Ok(filename) => filename,
        Err(_) => "/tmp/.xkb_lingvo".to_string(),
    };

    let language_file_path = Path::new(&lang_filename);
    if !language_file_path.exists() {
        fs::File::create(language_file_path).unwrap();
    }

    for event in Connection::new()?.subscribe([EventType::Input])? {
        match event? {
            Event::Input(event) => {
                if let Some(keyboard_lang) = event.input.xkb_active_layout_name {
                    if std::fs::read_to_string(&lang_filename)? != "keyboard_lang" {
                        fs::write(&lang_filename, keyboard_lang)?;
                    }
                }
            },
            _ => {}
        }
    }
    Ok(())
}
