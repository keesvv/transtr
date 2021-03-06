mod transfrm;

use std::{borrow::Borrow, convert::TryFrom, env::args, io::{Read, stdin}};
use transfrm::{TransformType, sequence::TransformSequence};

fn main() {
    let mut args: Vec<String> = args().collect();
    let mut input = String::new();
    let mut transfrm_types: Vec<TransformType> = Vec::new();

    args.drain(0..1);

    for arg in args.iter() {
        let transfrm_type: TransformType = TransformType::try_from(
            arg.as_str()
        ).unwrap();

        transfrm_types.push(transfrm_type);
    }

    stdin().read_to_string(&mut input).unwrap();
    input = input.trim().to_string();
 
    print!("{}", TransformSequence::new(transfrm_types.borrow())
        .transform_all(&input));
}
