use clap::{App, Arg};

fn main() {
    let app = App::new("gotcha")
        .version("0.1.0")
        .author("nnnamani <6526114+nnnamani@users.noreply.github.com>")
        .about("Random string generate tool")
        .arg(
            Arg::with_name("lowercase-letter")
                .short("a")
                .help("With lowercase letter [a - z]")
                .long("with-lowercase"),
        )
        .arg(
            Arg::with_name("uppercase-letter")
                .short("A")
                .help("With uppercase letter [A - Z]")
                .long("with-uppercase"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .help("With number string [0 - 9]")
                .long("with-number"),
        )
        .arg(
            Arg::with_name("symbol")
                .short("s")
                .help("With symbol [!#$%&'()*+,-./;<=>?@[]^_`{|}~]")
                .long("with-symbol"),
        )
        .arg(
            Arg::with_name("length")
                .short("l")
                .help("Length of generated string")
                .long("length")
                .value_name("LENGTH")
                .takes_value(true),
        );

    let matches = app.get_matches();

    let length = if let Some(o) = matches.value_of("length") {
        o.parse().unwrap()
    } else {
        10
    };

    let generator = gotcha::Generator::new(
        matches.is_present("number"),
        matches.is_present("lowercase-letter"),
        matches.is_present("uppercase-letter"),
        matches.is_present("symbol"),
    );

    let result = generator.generate_random_string(length);

    println!("{}", result.into_iter().collect::<String>());
}
