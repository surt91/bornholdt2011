mod model;
mod animate;

fn main() {
    let mut v = model::Bornholdt::new(64);
    animate::show(&mut v);
}
