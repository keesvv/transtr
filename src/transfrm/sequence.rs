use super::TransformType;

pub struct TransformSequence<'a> {
    types: &'a Vec<TransformType>
}

impl<'a> TransformSequence<'a> {
    pub fn new(types: &'a Vec<TransformType>) -> Self {
        TransformSequence { types }
    }

    pub fn transform_all(&self, s: &str) -> String {
        let mut output = String::from(s);

        for ttype in self.types.iter() {
            output = ttype.transform(output.as_str());
        }

        return output;
    }
}
