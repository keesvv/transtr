mod transfrm;

use std::{convert::TryFrom, env::args, io::{Read, stdin}};
use transfrm::TransformType;

fn main() {
    let mut args: Vec<String> = args().collect();
    let mut input = String::new();
    let mut output;
    let mut transfrm_types: Vec<TransformType> = Vec::new();

    args.drain(0..1);

    for arg in args.iter() {
        let transfrm_type: TransformType = TransformType::try_from(
            arg.as_str()
        ).unwrap();

        transfrm_types.push(transfrm_type);
    }

    stdin().read_to_string(&mut input).unwrap();
    output = input.clone();

    for ttype in transfrm_types.iter() {
        output = ttype.transform(output.as_str());
    }
 
    print!("{}", output.trim());
}
