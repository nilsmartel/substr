use std::env::args;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);

    let result = match (
        args().nth(1).and_then(|s| s.parse().ok()),
        args().nth(2).and_then(|s| s.parse().ok()),
    ) {
        (Some(from), Some(to)) => &input[from..to],

        (Some(until), None) => &input[0..until],
        _ => {
            eprintln!("substr requires one or two arguments in the form of numbers");
            std::process::exit(0);
        }
    };

    println!("{}", result)
}
