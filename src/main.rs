use encoding_rs::Encoding;
use std::io::BufRead;

fn main() {
    let mut args = std::env::args();
    let from = args.nth(1).unwrap_or_else(|| {
        println!("encoding name is not specified");
        std::process::exit(0);
    });
    let to = args.nth(2).unwrap_or("utf-8".to_string());

    let encoding_from = Encoding::for_label(from.as_bytes()).unwrap_or_else(|| {
        println!("{} is not recognized", from);
        std::process::exit(0);
    });

    let encoding_to = Encoding::for_label(to.as_bytes()).unwrap_or_else(|| {
        println!("{} is not recognized", to);
        std::process::exit(0);
    });

    let stdin = std::io::stdin();
    let mut reader = stdin.lock();

    loop {
        let mut bytes = Vec::new();
        match reader.read_until(b'\n', &mut bytes) {
            Ok(_) => {
                let line = String::from_utf8_lossy(&bytes);
                let file = line.trim();
                if file.len() > 0 {
                    convert_encoding(file, encoding_from, encoding_to);
                } else {
                    break;
                }
            }
            Err(error) => {
                println!("error: {}", error);
                break;
            }
        }
    }
}

fn convert_encoding(file: &str, encoding_from: &'static Encoding, encoding_to: &'static Encoding) {
    match std::fs::read(file) {
        Ok(bytes) => {
            let (string, encoding, has_malformed) = encoding_from.decode(&bytes);
            if encoding != encoding_from {
                println!("Detected encoding is {}: {}", encoding.name(), file);
            }
            if has_malformed {
                println!("There are malformed characters in {}", file);
            } else {
                let (bytes, encoding, has_unmappable) = encoding_to.encode(&string);
                if encoding != encoding_to {
                    println!("Saved encoding is {}: {}", encoding.name(), file);
                }
                if has_unmappable {
                    println!("There are unmappable characters in {}", file);
                }
                std::fs::write(file, bytes).unwrap_or_else(|err| {
                    println!("Can not write: {}", file);
                    println!("{:?}", err);
                });
            }
        }
        Err(err) => {
            println!("Can not read: {}", file);
            println!("{:?}", err);
        }
    }
}
