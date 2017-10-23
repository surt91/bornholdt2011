extern crate rand;
use self::rand::{Rng, SeedableRng};

use super::animate::Renderable;
use super::Model;

struct Agent {
    pub opinion: usize,
    pub neighbors: [usize; 4]
}

pub struct Neighbor {
    l: usize,
    alpha: f64,
    agents: Vec<Agent>,
    total_sweeps: usize,
    rng: rand::XorShiftRng,
}

impl Neighbor {
    pub fn new(l: usize, alpha: f64) -> Neighbor {
        let mut agents: Vec<Agent> = Vec::new();
        let mut rng = rand::XorShiftRng::from_seed([13, 42, 23, 15]);

        for i in 0..l {
            for j in 0..l {
                let mut neighbors = [0; 4]; // right, up, left, down
                neighbors[0] = if j == l-1 {i * l + 0} else {i*l + (j+1)};
                neighbors[1] = if i == 0 {(l-1) * l + j} else {(i-1)*l + j};
                neighbors[2] = if j == 0 {i * l + (l-1)} else {i*l + (j-1)};
                neighbors[3] = if i == l-1 {0 * l + j} else {(i+1)*l + j};

                let a = Agent {
                    opinion: rng.gen_range(0, 2),
                    neighbors
                };
                agents.push(a);
            }
        }

        Neighbor {
            l,
            alpha,
            agents,
            total_sweeps: 0,
            rng
        }
    }
}

impl Model for Neighbor {
    fn l(&self) -> usize {
        self.l
    }

    fn total_sweeps(&self) -> usize {
        self.total_sweeps
    }

    fn sweep(&mut self, number_of_sweeps: usize) {
        let total = self.l * self.l;

        for _ in 0..number_of_sweeps {
            self.total_sweeps += 1;
            for _ in 0..total {
                if self.rng.gen::<f64>() < self.alpha {
                    let idx = self.rng.gen_range(0, total);
                    self.agents[idx].opinion = self.rng.gen_range(0, 2);
                }

                // get random agent
                let idx = self.rng.gen_range(0, total);

                // get random neighbor
                let n1 = self.rng.gen_range(0, 4);
                let mut n2;
                while {
                    n2 = self.rng.gen_range(0, 4);
                    n1 == n2
                } {}

                if self.agents[self.agents[idx].neighbors[n1]].opinion == self.agents[self.agents[idx].neighbors[n2]].opinion
                {
                    // take its opinion
                    self.agents[idx].opinion = self.agents[self.agents[idx].neighbors[n1]].opinion;
                }
            }
        }
    }
}


use super::graphics::*;
use super::animate::opinion_to_color;
impl Renderable for super::Neighbor {
    fn render<G>(&self, c: &Context, gfx: &mut G, _size: &(u32, u32))
        where G: Graphics
    {
        clear(color::hex("000000"), gfx);
        for i in 0..self.l {
            for j in 0..self.l {
                rectangle(opinion_to_color(self.agents[i*self.l+j].opinion),
                          rectangle::square(i as f64 * 5.,
                                            j as f64 * 5.,
                                            5.),
                          c.transform, gfx
                );
            }
        }
    }
}
