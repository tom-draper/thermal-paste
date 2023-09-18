use clipboard_win::{formats, get_clipboard};

fn main() {
    let result: String = get_clipboard(formats::Unicode).unwrap_or_default();
    println!("{}", result)
}
