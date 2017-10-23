extern crate graphics;

mod bornholdt;
mod simple;
pub mod animate;

pub use self::bornholdt::Bornholdt;
pub use self::simple::Simple;

pub trait Model {
    fn sweep(&mut self, number_of_sweeps: usize);
    fn l(&self) -> usize;
    fn total_sweeps(&self) -> usize;
}
