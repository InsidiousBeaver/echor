use clap::{Arg, Command};

fn main() {
    let mut app = Command::new("echor")
        .name("echor")
        .author("Insidious Beaver")
        .version("0.1.0")
        .about("Rust echo")
        .long_about(None);

    let text = Arg::new("text")
        .value_name("TEXT")
        .help("Input text")
        .required(true)
        .min_values(1);

    let omit_newline = Arg::new("omit_newline")
        .short('n')
        .help("Do not print newline")
        .takes_value(false);

    app = app.arg(text).arg(omit_newline);

    let matches = app.get_matches();
    let text: Vec<String> = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|val| val.to_string())
        .collect();
    let omit_newline = matches.contains_id("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
