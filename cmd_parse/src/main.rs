use clap::{Arg, App};

// this type of parsing uses a crate, parsing can be done without a crate
// see the tinymd rust project for how that is accomplished
fn main() {
    let matches = App::new("Tester Program")
        .version("0.1.0")
        .author("Cody Diamond <Email@email.com>")
        .about("Just a little test program to play with the clap CLI builder")
        .arg(Arg::with_name("file").short('f').long("file").takes_value(true).help("A cool file."))
        .arg(Arg::with_name("num").short('n').long("number").takes_value(true).help("Five less than your favorite number."))
        .get_matches();

    let my_file = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", my_file);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No clue what your favorite number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }

}
