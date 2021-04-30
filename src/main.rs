mod transfrm;

use std::{convert::TryFrom, env::args, io::{Read, stdin}};
use transfrm::TransformType;

fn main() {
    let args: Vec<String> = args().collect();
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let transfrm_type: TransformType = TransformType::try_from(
        args[1].as_str()
    ).unwrap();

    print!("{}", transfrm_type.transform(&input).trim());
}
