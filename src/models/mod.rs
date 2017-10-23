extern crate graphics;

mod simple;
mod noise;
mod neighbor;
mod bornholdt;

pub mod animate;

pub use self::simple::Simple;
pub use self::noise::Noise;
pub use self::neighbor::Neighbor;
pub use self::bornholdt::Bornholdt;

pub trait Model {
    fn sweep(&mut self, number_of_sweeps: usize);
    fn l(&self) -> usize;
    fn total_sweeps(&self) -> usize;
}
