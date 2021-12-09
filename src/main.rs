mod models;
use models::animate;

mod parse_cl;
use parse_cl::{parse_cl, ModelType};



fn main() {
    let options = parse_cl();

    println!("Some things can be controlled at run time:");
    println!("F: show current framerate");
    println!("S: show number of sweeps");
    println!("P: ??");
    println!("Up: increase sweeps per second");
    println!("Down: decrese sweeps per second");

    match options.model_type {
        ModelType::Simple => {
            let mut v = models::Simple::new(options.length);
            animate::show(&mut v);
        },
        ModelType::Noise => {
            let mut v = models::Noise::new(options.length, options.alpha);
            animate::show(&mut v);
        },
        ModelType::Neighbor => {
            let mut v = models::Neighbor::new(options.length, options.alpha);
            animate::show(&mut v);
        },
        ModelType::Bornholdt => {
            let mut v = models::Bornholdt::new(options.length, options.alpha);
            animate::show(&mut v);
        }
    }

}
