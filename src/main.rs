use std::{
    fs::File,
    io::{self, Read, Write, stdin},
    path::PathBuf,
    str::FromStr,
};

use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command, ValueHint, builder::Str};

pub mod art_encoding;

enum InputMethod {
    InputFile(PathBuf),
    InputHex(Vec<u8>),
    InputText(String),
    InputImage(String),
}
#[derive(Clone, Copy)]
pub struct AppFlags {
    silent: bool,
}

pub struct EncodingFlags {
    app_flags: AppFlags,
    skip_length_check: bool,
}

fn encode_bytes(bytes: &[u8], encoding_flags: &EncodingFlags) -> String {
    if !encoding_flags.app_flags.silent {
        println!("Generating image...");
    }
    let img = match art_encoding::gen_encoding(&bytes, encoding_flags.skip_length_check) {
        Ok(ok) => ok,
        Err(e) => {
            if !encoding_flags.app_flags.silent {
                println!("Couldn't generate image: {e}");
            }
            std::process::exit(16);
        }
    };
    return img;
}

fn read_image_stdin(flags: &AppFlags) -> Result<String, io::Error> {
    let mut buf = String::new();
    if !flags.silent {
        println!("Enter the image");
    }
    let mut lines = 0;
    let mut grid_size = 0;
    let mut img = String::new();
    loop {
        buf.clear();
        let res = stdin().read_line(&mut buf)?;

        lines += 1;
        if grid_size == 0 {
            if !buf.starts_with('╭') {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid image. Expected image beginning",
                ));
            }
            grid_size = buf.chars().count() / 2 - 1;
        }
        if lines == grid_size + 2 {
            if !buf.starts_with("╰") {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid image. Expected image end",
                ));
            }
            break;
        }
        img.push_str(&buf);
    }
    return Ok(img);
}

fn decode_image(img: String, flags: &AppFlags) -> Vec<u8> {
    if !flags.silent {
        println!("Decoding image...");
    }
    art_encoding::decode_image(&img)[1..].to_vec()
}

fn encode_subcommand() -> Command {
    Command::new("encode")
    .about("Encode some data into an artwork")
    .groups([
        ArgGroup::new("input_method")
        .args(["in_file", "in_hex", "in_text"])
        .required(true)
        .multiple(false)
    ])
    .args([
        //Input methods
        Arg::new("in_file")
            .help("Specify that the input is a file")
            .action(ArgAction::SetTrue)
            .long("file")
            .short('f'),
        Arg::new("in_hex")
            .help("Specify that the input is a hex string")
            .action(ArgAction::SetTrue)
            .long("hex")
            .short('x'),
        Arg::new("in_text")
            .help("Specify that the input is a utf-8 string")
            .action(ArgAction::SetTrue)
            .long("text")
            .short('t'),

        //Flags
        Arg::new("skip_length_check")
            .help("Allow infinite length inputs")
            .long("skip_length_check")
            .action(ArgAction::SetTrue),


        //IO
        Arg::new("input")
            .help("Specify an input.")
            .long_help("Specify an input. The type of this input is given by the input method.")
            .short('i')
            .long("input")
            .index(1)
            .required(true),
        //If output is not set, outputs will be given over stdout
        Arg::new("output")
            .help("Specify an output file.")
            .long("Specify an output file. If this is left empty, operation outputs will be given over stdout.")
            .short('o')
            .long("output")
            .index(2)
            .required(false),
    ])
}

fn handle_encode(matches: &ArgMatches, app_flags: &AppFlags) {
    let encoding_flags = EncodingFlags {
        app_flags: *app_flags,
        skip_length_check: matches.get_flag("skip_length_check"),
    };
    let input = matches.get_one::<String>("input").unwrap();
    let mut byte_vec;
    let input_bytes;
    if matches.get_flag("in_hex") {
        byte_vec = match hex::decode(input) {
            Ok(ok) => ok,
            Err(e) => {
                if app_flags.silent {
                    println!("Couldn't parse the hex string: {e}");
                }
                std::process::exit(16);
            }
        };
        input_bytes = byte_vec.as_slice();
    } else if matches.get_flag("in_text") {
        input_bytes = input.as_bytes();
    } else if matches.get_flag("in_file") {
        let mut file = match File::open(input) {
            Ok(ok) => ok,
            Err(e) => {
                if !app_flags.silent {
                    println!("Couldn't open file: {e}");
                }
                std::process::exit(16);
            }
        };
        byte_vec = vec![];
        if let Err(e) = file.read_to_end(&mut byte_vec) {
            if !app_flags.silent {
                println!("Couldn't read file: {e}");
            }
            std::process::exit(16);
        }
        input_bytes = byte_vec.as_slice();
    } else {
        panic!("No input method provided");
    }

    let image = encode_bytes(input_bytes, &encoding_flags);

    let output = matches.get_one::<String>("output");
    match output {
        Some(p) => {
            let mut file = match File::create(p) {
                Ok(ok) => ok,
                Err(e) => {
                    if !app_flags.silent {
                        println!("Couldn't create file: {e}");
                    }
                    std::process::exit(16);
                }
            };
            if let Err(e) = file.write_all(image.as_bytes()) {
                if !app_flags.silent {
                    println!("Couldn't write to file: {e}");
                }
                std::process::exit(16);
            }
        }
        None => {
            if app_flags.silent {
                println!("{image}")
            } else {
                println!("Your image for '{input}':\n{image}")
            }
        }
    }
}

fn decode_subcommand() -> Command {
    Command::new("decode")
    .about("Decode an artwork into some bytes")

    .args([
        //IO
        Arg::new("input")
            .help("Specify an input.")
            .long_help("Specify an input file. If no file is given, you will be asked at runtime to provide an image over the standard input.")
            .index(1)
            .short('i')
            .long("input")
            .required(false),
        //If output is not set, outputs will be given over stdout
        Arg::new("output")
            .help("Specify an output file.")
            .long_help("Specify an output file. If this is left empty, operation outputs will be given over stdout.")
            .index(2)
            .short('o')
            .long("output")
            .required(false),
    ])
}

fn handle_decode(matches: &ArgMatches, app_flags: &AppFlags) {
    let input_img = match matches.get_one::<PathBuf>("input") {
        Some(path) => {
            let mut file = match File::open(path) {
                Ok(ok) => ok,
                Err(e) => {
                    if !app_flags.silent {
                        println!("Couldn't open file: {e}");
                    }
                    std::process::exit(16);
                }
            };
            let mut buf = String::new();
            if let Err(e) = file.read_to_string(&mut buf) {
                if !app_flags.silent {
                    println!("Couldn't read file: {e}");
                }
                std::process::exit(16);
            }
            buf
        }
        None => match read_image_stdin(app_flags) {
            Ok(ok) => ok,
            Err(e) => {
                if !app_flags.silent {
                    println!("Couldn't read image: {e}");
                }
                std::process::exit(16);
            }
        },
    };

    let image = decode_image(input_img, app_flags);

    let output = matches.get_one::<String>("output");
    match output {
        Some(p) => {
            let mut file = match File::create(p) {
                Ok(ok) => ok,
                Err(e) => {
                    if !app_flags.silent {
                        println!("Couldn't create file: {e}");
                    }
                    std::process::exit(16);
                }
            };
            if let Err(e) = file.write_all(&image) {
                if !app_flags.silent {
                    println!("Couldn't write to file: {e}");
                }
                std::process::exit(16);
            }
        }
        None => {
            if app_flags.silent {
                println!("{}", hex::encode(image));
                return;
            }
            println!("As hex: {}", hex::encode(&image));

            if let Ok(s) = String::from_utf8(image) {
                println!("As utf8: '{s}'")
            }
        }
    }
}

fn main() {
    let command = Command::new("One time share")
        .about("Generate an artwork that encodes data")
        .arg(
            Arg::new("silent")
            .help("Disable all standard output except necessary.")
            .long_help("Disable all standard output except necessary. There will only be output over standard output if no output file is provided and the program has an operation result.")
            .short('s')
            .long("silent")
            .action(ArgAction::SetTrue)
            .global(true)
        )

        .subcommand(encode_subcommand())
        .subcommand(decode_subcommand())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .after_help("Go to https://github.com/Ekischleki/key_art for source code and more information")
        .get_matches();

    let app_flags = AppFlags {
        silent: command.get_flag("silent"),
    };

    match command.subcommand() {
        Some(("decode", subcommand)) => handle_decode(subcommand, &app_flags),
        Some(("encode", subcommand)) => handle_encode(subcommand, &app_flags),
        _ => panic!("No subcommand given"),
    }

    //let decode = command.get_one("decode_image").unwrap()
}
