extern crate rand;
use self::rand::{Rng, SeedableRng};

use std::collections::HashSet;

pub struct Agent {
    pub opinion: usize,
    pub old_opinions: HashSet<usize>,
    pub neighbors: [usize; 4]
}

pub struct Bornholdt {
    pub l: usize,
    pub alpha: f64,
    pub agents: Vec<Agent>,
    pub opinions: Vec<usize>,
    newest_opinion: usize,
    pub total_sweeps: usize,
    rng: rand::XorShiftRng,
}

impl Bornholdt {
    pub fn new(l: usize, alpha: f64) -> Bornholdt {
        let mut agents: Vec<Agent> = Vec::new();
        for i in 0..l {
            for j in 0..l {
                let mut neighbors = [0; 4]; // right, up, left, down
                neighbors[0] = if j == l-1 {i * l + 0} else {i*l + (j+1)};
                neighbors[1] = if i == 0 {(l-1) * l + j} else {(i-1)*l + j};
                neighbors[2] = if j == 0 {i * l + (l-1)} else {i*l + (j-1)};
                neighbors[3] = if i == l-1 {0 * l + j} else {(i+1)*l + j};

                let mut old_opinions = HashSet::new();
                old_opinions.insert(0);

                let a = Agent {
                    opinion: 0,
                    old_opinions,
                    neighbors
                };
                agents.push(a);
            }
        }

        let opinions = vec![l*l]; // initial condition: everyone has same opinion
        let newest_opinion = 0;
        let rng = rand::XorShiftRng::from_seed([13, 42, 23, 15]);

        Bornholdt {
            l,
            alpha,
            agents,
            opinions,
            newest_opinion,
            total_sweeps: 0,
            rng
        }
    }

    pub fn sweep(&mut self, number_of_sweeps: usize) {
        let N = self.l * self.l;

        // clean up
        let mut free_numbers: Vec<usize> = self.opinions.iter()
                                                        .enumerate()
                                                        .filter(|&(_, &o)| o == 0)
                                                        .map(|(n, _)| n)
                                                        .collect();

        for _ in 0..number_of_sweeps {
            self.total_sweeps += 1;
            for _ in 0..N {
                // get random agent
                let idx = self.rng.gen_range(0, N);

                // with chance alpha, make an innovation on a random agent
                if self.rng.gen::<f64>() < self.alpha {
                    // try to recycle old numbers
                    self.newest_opinion = match free_numbers.pop() {
                        Some(x) => {
                            self.opinions[x] = 1;
                            for a in self.agents.iter_mut() {
                                a.old_opinions.remove(&x);
                            };
                            x
                        },
                        None => {
                            self.opinions.push(1);
                            self.opinions.len() - 1
                        }
                    };
                    self.opinions[self.agents[idx].opinion] -= 1;
                    self.agents[idx].opinion = self.newest_opinion;
                    continue
                }

                // get random neighbor
                let n = self.rng.gen_range(0, 4);
                let opinion = self.agents[self.agents[idx].neighbors[n]].opinion;

                let agent = &mut self.agents[idx];

                // do not take an old opinion
                if agent.old_opinions.contains(&opinion) {
                    continue
                }

                // take its opinion with chance n_j / N
                let chance = self.opinions[opinion] as f64 / N as f64;

                if self.rng.gen::<f64>() < chance {
                    // println!("extend!");
                    agent.old_opinions.insert(opinion);
                    self.opinions[agent.opinion] -= 1;
                    self.opinions[opinion] += 1;
                    agent.opinion = opinion;
                }
            }
        }
    }
}
