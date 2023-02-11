extern crate clap;
use clap::{Arg, Command, ArgAction};

fn main() {
    let matches = Command::new("word_salad")
        .version("1.0")
        .bin_name("word_salad")
        .author("Edgar H. Sanchez <esanchez@m2iab.com>")
        .about("Generates random words from a key source")
        .arg(
            Arg::new("key-file")
                .short('k')
                .long("key-file")
                .action(ArgAction::Set))
        .get_matches();

    if let Some(key_file) = matches.get_one::<String>("key-file") {
        println!("{}", key_file);
    }
}
