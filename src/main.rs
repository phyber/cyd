// cyd: convert your data
//
// Takes a document on STDIN, converts it to the desired format and outputs on
// STDOUT.
//
// We cannot use serde_transcode since all formats do not export the
// Serializer and Deserializer types.
// Similarly, we cannot use from_reader and to_writer with every format as some
// do not have those methods.
//
use std::io;
use std::io::prelude::*;

// Macro using crates
#[macro_use]
extern crate clap;

// VALID_FORMATS is an array of possible formats we can convert to/from, used
// in the clap argument parsing phase.
// Values are matched case-insensitively.
static VALID_FORMATS: &'static [&'static str] = &[
    "json", // serde_json
    "toml", // toml
    "yaml", // serde_yaml
];

// An enum holding our return values from the input data.
#[derive(Debug)]
enum InputData {
    JSON(serde_json::Value),
    TOML(toml::Value),
    YAML(serde_yaml::Value),
}

impl InputData {
    fn to_json(&self, out: impl Write) -> Result<(), serde_json::Error> {
        match self {
            InputData::JSON(v) => serde_json::to_writer(out, &v),
            InputData::TOML(v) => serde_json::to_writer(out, &v),
            InputData::YAML(v) => serde_json::to_writer(out, &v),
        }
    }

    fn to_toml(&self, mut out: impl Write) -> Result<(), toml::ser::Error> {
        // The toml crate doesn't support the to_writer method, so output is a
        // little more verbose here.
        let output = match self {
            InputData::JSON(v) => toml::to_string(&v),
            InputData::TOML(v) => toml::to_string(&v),
            InputData::YAML(v) => toml::to_string(&v),
        };

        match output {
            Ok(s) => {
                write!(out, "{}", s);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    fn to_yaml(&self, out: impl Write) -> Result<(), serde_yaml::Error> {
        match self {
            InputData::JSON(v) => serde_yaml::to_writer(out, &v),
            InputData::TOML(v) => serde_yaml::to_writer(out, &v),
            InputData::YAML(v) => serde_yaml::to_writer(out, &v),
        }
    }
}

fn main() {
    let app = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            clap::Arg::with_name("INPUT")
            .help("Specify format of input document")
            .required(true)
            .long("input")
            .short("i")
            .env("CYD_INPUT")
            .hide_env_values(true)
            .value_name("FORMAT")
            .takes_value(true)
            .case_insensitive(true)
            .possible_values(VALID_FORMATS)
        )
        .arg(
            clap::Arg::with_name("OUTPUT")
            .help("Specify format of output document")
            .required(true)
            .long("output")
            .short("o")
            .env("CYD_OUTPUT")
            .hide_env_values(true)
            .value_name("FORMAT")
            .takes_value(true)
            .case_insensitive(true)
            .possible_values(VALID_FORMATS)
        );

    let matches = app.get_matches();

    // Unwraps should be safe, we require these args above.
    let input_type = matches.value_of("INPUT").unwrap();
    let output_type = matches.value_of("OUTPUT").unwrap();

    // Get our input and output streams
    // stdin needs to be mut for TOML input
    let mut stdin = io::stdin();
    let stdout = io::stdout();

    // Process input
    let input = match input_type.to_lowercase().as_ref() {
        "json" => {
            let de = serde_json::from_reader(stdin);
            match de {
                Ok(v) => InputData::JSON(v),
                Err(e) => {
                    eprintln!("error: {}", e);
                    ::std::process::exit(1);
                },
            }
        },
        "toml" => {
            // No from_reader here.
            let mut s = String::new();
            stdin.read_to_string(&mut s).expect("couldn't read stdin");

            let de = toml::from_str(&s);
            match de {
                Ok(v) => InputData::TOML(v),
                Err(e) => {
                    eprintln!("error: {}", e);
                    ::std::process::exit(1);
                },
            }
        },
        "yaml" => {
            let de = serde_yaml::from_reader(stdin);
            match de {
                Ok(v) => InputData::YAML(v),
                Err(e) => {
                    eprintln!("error: {}", e);
                    ::std::process::exit(1);
                },
            }
        },
        _ => {
            // We shouldn't ever get here, since clap checks for input/output
            // types that we support.
            eprintln!("Error: deserialization");
            ::std::process::exit(1);
        },
    };

    // Output is slightly easier, we can just convert and dump to stdout right
    // away.
    match output_type.to_lowercase().as_ref() {
        "json" => {
            match input.to_json(stdout) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("JSON Error: {}", e);
                    ::std::process::exit(1);
                },
            }
        },
        "toml" => {
            match input.to_toml(stdout) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("TOML Error: {}", e);
                    ::std::process::exit(1);
                },
            }
        },
        "yaml" => {
            match input.to_yaml(stdout) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("YAML Error: {}", e);
                    ::std::process::exit(1);
                },
            }
        },
        _ => {
            // We shouldn't ever get here, since clap checks for input/output
            // types that we support.
            eprintln!("error: Unrecognised output format");
            ::std::process::exit(1);
        },
    };
}
