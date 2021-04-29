use std::{convert::TryFrom, env::args, io::{Read, stdin}};

const WIDTH: usize = 1;

enum ConvType {
    Wide
}

impl TryFrom<&str> for ConvType {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        return match s {
            "wide" => Ok(Self::Wide),
            _ => Err("converter does not exist")
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let mut input = String::new();
    let mut output = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let _conv_type: ConvType = ConvType::try_from(
        args[1].as_str()
    ).unwrap();

    for c in input.as_bytes() {
        output.push_str(
            String::from(
                format!("{}{}", *c as char, " ".repeat(WIDTH))
            ).as_str()
        )
    }

    print!("{}", output.trim());
}
