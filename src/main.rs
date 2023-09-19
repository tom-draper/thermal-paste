use clipboard_win::{formats, get_clipboard};
use hex;
use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Settings {
    append: bool,
    hex: bool,
    binary: bool,
    uppercase: bool,
    lowercase: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            append: false,
            hex: false,
            binary: false,
            uppercase: false,
            lowercase: false,
        }
    }
}

fn get_settings(args: &Vec<String>) -> Settings {
    let mut settings = Settings::default();
    for arg in args.iter() {
        match arg.as_str() {
            "-a" | "--append" => settings.append = true,
            "-h" | "--hex" => settings.hex = true,
            "-b" | "--bin" | "--binary" => settings.binary = true,
            _ => (),
        }
    }
    settings
}

fn get_path(args: &Vec<String>) -> Option<String> {
    for arg in args.iter().skip(1) {
        if arg.contains(".") || !arg.contains("-") {
            return Some(arg.to_string())
        }
    }
    None
}

fn main() {
    let mut contents: String = get_clipboard(formats::Unicode).unwrap_or_default();

    let args: Vec<String> = env::args().collect();
    let settings = get_settings(&args);
    let path = get_path(&args).unwrap_or_default();

    println!("{}", path);

    let mut file = File::create(path).unwrap();
    if settings.hex {
        contents = hex::encode(contents);
    } else if settings.binary {
        let mut binary_contents = String::new();
        for character in contents.clone().into_bytes() {
            binary_contents += &format!("0{:b}", character);
        }
        contents = binary_contents;
    } else if settings.uppercase {
        contents = contents.to_uppercase()
    } else if settings.lowercase {
        contents = contents.to_lowercase()
    }
    file.write_all(contents.as_bytes()).unwrap();
}
