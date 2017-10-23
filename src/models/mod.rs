extern crate graphics;

mod bornholdt;
mod simple;
mod noise;
pub mod animate;

pub use self::bornholdt::Bornholdt;
pub use self::simple::Simple;
pub use self::noise::Noise;

pub trait Model {
    fn sweep(&mut self, number_of_sweeps: usize);
    fn l(&self) -> usize;
    fn total_sweeps(&self) -> usize;
}
