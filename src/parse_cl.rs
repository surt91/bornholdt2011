extern crate clap;

use self::clap::{App, Arg};

#[derive(Debug)]
pub enum ModelType {
    Simple,
    Noise,
    Neighbor,
    Bornholdt
}

#[derive(Debug)]
pub struct Options {
    pub seed: usize,
    pub alpha: f64,
    pub length: usize,
    pub model_type: ModelType,
}

pub fn parse_cl() -> Options {
    let matches = App::new(env!("CARGO_PKG_NAME"))
              .version(env!("CARGO_PKG_VERSION"))
              .about(env!("CARGO_PKG_DESCRIPTION"))
              .author(env!("CARGO_PKG_AUTHORS"))
              .arg(Arg::with_name("simple")
                    .long("simple")
                    .help("simulate simple model")
                    .conflicts_with("bornholdt")
                    .conflicts_with("neighbor")
                    .conflicts_with("noise")
              )
              .arg(Arg::with_name("bornholdt")
                    .long("bornholdt")
                    .help("simulate bornholdt model")
                    .conflicts_with("simple")
                    .conflicts_with("neighbor")
                    .conflicts_with("noise")
              )
              .arg(Arg::with_name("neighbor")
                    .long("neighbor")
                    .help("simulate neighbor model")
                    .conflicts_with("bornholdt")
                    .conflicts_with("simple")
                    .conflicts_with("noise")
              )
              .arg(Arg::with_name("noise")
                    .long("noise")
                    .help("simulate noise model")
                    .conflicts_with("bornholdt")
                    .conflicts_with("neighbor")
                    .conflicts_with("simple")
              )
              .arg(Arg::with_name("alpha")
                    .short("a")
                    .long("alpha")
                    .takes_value(true)
                    .help("innovations rate")
              )
              .arg(Arg::with_name("length")
                    .short("l")
                    .long("length")
                    .takes_value(true)
                    .help("side length")
              )
              .arg(Arg::with_name("seed")
                    .short("s")
                    .long("seed")
                    .takes_value(true)
                    .help("random seed")
              )
              .get_matches();

    let seed = matches.value_of("seed")
                      .and_then(|s| Some(s.parse::<usize>().expect("seed needs to be an integer")))
                      .or_else(|| Some(42))
                      .unwrap();

    let length = matches.value_of("length")
                      .and_then(|s| Some(s.parse::<usize>().expect("length needs to be an integer")))
                      .or_else(|| Some(100))
                      .unwrap();

    let alpha = matches.value_of("alpha")
                      .and_then(|s| Some(s.parse::<f64>().expect("alpha needs to be an float")))
                      .or_else(|| Some(2e-5))
                      .unwrap();

    let model_type =
        if matches.is_present("simple") {
            ModelType::Simple
        } else if matches.is_present("noise") {
            ModelType::Noise
        } else if matches.is_present("neighbor") {
            ModelType::Neighbor
        } else if matches.is_present("bornholdt") {
            ModelType::Bornholdt
        } else {
            ModelType::Bornholdt
        };

    Options {
        seed,
        alpha,
        length,
        model_type
    }
}
