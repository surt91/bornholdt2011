mod model;
mod animate;

fn main() {
    let mut v = model::Bornholdt::new(128, 25e-6);
    animate::show(&mut v);
}
