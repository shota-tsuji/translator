use translator::LowerProcesser;
use clap::{App, Arg};

fn main() {
    let matches = App::new("translator-cli")
        .version("0.0.1")
        .author("Shota TSUJI")
        .about("translate a alphabetical word")
        .arg(Arg::with_name("word")
             .value_name("WORD")
             .help("an input word")
             .required(true)
             .index(1))
        .get_matches();
    
    let processor = LowerProcesser::new();

    if let Some(word) = matches.value_of("word") {
        println!("{}", processor.run(word));
    }
}