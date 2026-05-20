use std::io::stdin;

use clap::{Arg, ArgAction, Command};

pub mod art_encoding;

fn main() {
    let command = Command::new("One time share")
        .about("Generate an artwork that encodes data")
        .arg(
            Arg::new("encode_bytes")
                .help("Encode a hex string into an ascii artwork")
                .long_help("Encode a hex string into an ascii artwork. The hex string may not be greater than 64 bytes")
                .short('e'),
        )
        .arg(
            Arg::new("encode_text")
                .help("Encode a text string into an ascii artwork")
                .short('t')
                .conflicts_with("encode_bytes"),

        )
        .arg(
            Arg::new("silent")
                .action(ArgAction::SetTrue)
                .help("Don't print any status messages except absolutely necessary")
                .short('s')
        )
        .arg(
            Arg::new("decode_image")
                .short('d')
                .action(ArgAction::SetTrue)
                .long_help("Decode an ascii image back into the bytes that were used to produce it. This argument does not take a value and an image needs to be piped into stdin or simply entered when executing this command")
                .conflicts_with("encode_bytes")
                .conflicts_with("encode_text")
        )
        .get_matches();
    let silent = command.get_flag("silent");
    if let Some(t) = command.get_one::<String>("encode_text") {
        if !silent {
            println!("Generating image...");
        }
        let img = match art_encoding::gen_encoding(t.as_bytes()) {
            Ok(ok) => ok,
            Err(e) => {
                if silent {
                    std::process::exit(16);
                }
                println!("Couldn't generate image: {e}");
                return;
            }
        };
        if silent {
            println!("{img}");
        } else {
            println!("Your image for '{t}':\n{img}");
        }
        return;
    }
    if let Some(b) = command.get_one::<String>("encode_bytes") {
        let bytes = match hex::decode(b) {
            Ok(ok) => ok,
            Err(e) => {
                if silent {
                    std::process::exit(16);
                }
                println!("Couldn't parse the hex string: {e}");
                return;
            }
        };
        if !silent {
            println!("Generating image...");
        }
        let img = match art_encoding::gen_encoding(&bytes) {
            Ok(ok) => ok,
            Err(e) => {
                if silent {
                    std::process::exit(16);
                }
                println!("Couldn't generate image: {e}");
                return;
            }
        };
        if silent {
            println!("{img}");
        } else {
            println!("Your image for '{b}':\n{img}");
        }
        return;
    }
    if command.get_flag("decode_image") {
        let mut buf = String::new();
        if !silent {
            println!("Enter the image");
        }
        let mut lines = 0;
        let mut grid_size = 0;
        let mut img = String::new();
        loop {
            buf.clear();
            let res = stdin().read_line(&mut buf);
            if let Err(e) = res {
                if silent {
                    std::process::exit(16);
                }
                println!("Couldn't read from stdin: {e}");
                return;
            }
            lines += 1;
            if grid_size == 0 {
                if !buf.starts_with('╭') {
                    if silent {
                        std::process::exit(16);
                    }
                    println!("Invalid image. Expected image beginning");
                    return;
                }
                grid_size = buf.chars().count() / 2 - 1;
            }
            if lines == grid_size + 2 {
                if !buf.starts_with("╰") {
                    if silent {
                        std::process::exit(16);
                    }
                    println!("Invalid image. Expected image end");
                    return;
                }
                break;
            }
            img.push_str(&buf);
        }
        let bytes = art_encoding::decode_image(&img)[1..].to_vec();

        if silent {
            println!("{}", hex::encode(bytes));
            return;
        }
        println!("As hex: {}", hex::encode(&bytes));

        if let Ok(s) = String::from_utf8(bytes) {
            println!("As utf8: '{s}'")
        }
    }
    //let decode = command.get_one("decode_image").unwrap()
}
