extern crate rand;
use self::rand::{Rng, SeedableRng};

use super::animate::Renderable;
use super::Model;
use super::graphics::*;
use super::animate::opinion_to_color;
use super::square_neighbors;

struct Agent {
    pub opinion: usize,
    pub neighbors: [usize; 4]
}

pub struct Noise {
    l: usize,
    alpha: f64,
    agents: Vec<Agent>,
    total_sweeps: usize,
    rng: rand::XorShiftRng,
}

impl Noise {
    pub fn new(l: usize, alpha: f64) -> Noise {
        let mut agents: Vec<Agent> = Vec::new();
        let mut rng = rand::XorShiftRng::from_seed([13, 42, 23, 15]);

        for i in 0..l {
            for j in 0..l {
                let neighbors = square_neighbors(i, j, l);

                let a = Agent {
                    opinion: rng.gen_range(0, 2),
                    neighbors
                };
                agents.push(a);
            }
        }

        Noise {
            l,
            alpha,
            agents,
            total_sweeps: 0,
            rng
        }
    }
}

impl Model for Noise {
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
                let n = self.rng.gen_range(0, 4);
                let opinion = self.agents[self.agents[idx].neighbors[n]].opinion;

                let agent = &mut self.agents[idx];

                // take its opinion
                agent.opinion = opinion;
            }
        }
    }
}

impl Renderable for super::Noise {
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
