mod models;
use models::animate;

mod parse_cl;
use parse_cl::{parse_cl, ModelType};



fn main() {
    let options = parse_cl();
    match options.model_type {
        ModelType::Simple => {
            let mut v = models::Simple::new(options.length);
            animate::show(&mut v);
        },
        ModelType::Noise => unimplemented!(),
        ModelType::Neighbor => unimplemented!(),
        ModelType::Bornholdt => {
            let mut v = models::Bornholdt::new(options.length, options.alpha);
            animate::show(&mut v);
        }
    }

}
