use ansi_term::Colour;
use ansi_term::Style;

fn main() {
    // example of using ansi_term to color and style some basic text
    println!("This is {} in color, {} in color and {} in color.",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green"));

    println!("{} and this is not.",
        Style::new().bold().paint("This is Bold"));

    println!("{}, {} and {}.",
        Colour::Yellow.paint("This is colored"),
        Style::new().bold().paint("This is bold"),
        Colour::Yellow.bold().paint("This is bold and colored"));
}
