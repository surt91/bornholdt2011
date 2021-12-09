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

fn square_neighbors(i: usize, j: usize, l: usize) -> [usize ;4] {
    let mut neighbors = [0; 4]; // right, up, left, down
    neighbors[0] = if j == l-1 {i * l} else {i*l + (j+1)};
    neighbors[1] = if i == 0 {(l-1) * l + j} else {(i-1)*l + j};
    neighbors[2] = if j == 0 {i * l + (l-1)} else {i*l + (j-1)};
    neighbors[3] = if i == l-1 {j} else {(i+1)*l + j};

    neighbors
}