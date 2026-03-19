use anyhow::Result;
use std::env;
use std::fs;
use std::path::Path;

mod ipc;

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

fn write_layout_if_changed(path: &str, layout: &str) -> Result<()> {
    if fs::read(path).unwrap_or_default() != layout.as_bytes() {
        fs::write(path, layout)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let lang_filename = get_lang_filename_from_args();

    // Ensure file exists.
    let language_file_path = Path::new(&lang_filename);
    if !language_file_path.exists() {
        fs::File::create(language_file_path)?;
    }

    let mut input_stream = ipc::InputStream::connect()?;

    if let Some(layout) = input_stream.current_layout()? {
        write_layout_if_changed(&lang_filename, &layout)?;
    }

    loop {
        if let Some(layout) = input_stream.next_layout_change()? {
            write_layout_if_changed(&lang_filename, &layout)?;
        }
    }
}
