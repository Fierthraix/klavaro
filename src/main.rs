use std::env;
use std::fs;
use std::path::Path;
use swayipc::{Connection, Event, EventType, Fallible};

const KLAVARO_FILE: &str = "/tmp/.xkb_lingvo";

const HELP: &str = r#"
Print the current xkb_layout in sway.
The default output file is `/tmp/.xkb_lingvo'
USAGE:
    klavaro [OUTPUT_FILE]
"#;

fn get_lang_filename_from_args() -> String {
    if env::args().any(|arg| arg == "-h" || arg == "--help") {
        print!("{}", HELP);
        std::process::exit(0);
    }

    if let Some(filename) = env::args().nth(1) {
        filename
    } else {
        KLAVARO_FILE.to_string()
    }
}

fn main() -> Fallible<()> {
    let lang_filename = get_lang_filename_from_args();

    // Ensure file exists.
    let language_file_path = Path::new(&lang_filename);
    if !language_file_path.exists() {
        fs::File::create(language_file_path).unwrap();
    }

    // Watch for events.
    for event in Connection::new()?.subscribe([EventType::Input])? {
        if let Event::Input(event) = event? {
            if let Some(keyboard_lang) = event.input.xkb_active_layout_name {
                if std::fs::read_to_string(&lang_filename)? != "keyboard_lang" {
                    fs::write(&lang_filename, keyboard_lang)?;
                }
            }
        }
    }
    Ok(())
}
