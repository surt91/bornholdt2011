mod model;
mod animate;

mod parse_cl;
use parse_cl::{parse_cl, ModelType};

fn main() {
    let options = parse_cl();
    match options.model_type {
        ModelType::Simple => unimplemented!(),
        ModelType::Noise => unimplemented!(),
        ModelType::Neighbor => unimplemented!(),
        ModelType::Bornholdt => {
            let mut v = model::Bornholdt::new(options.length, options.alpha);
            animate::show(&mut v);
        }
    }

}
