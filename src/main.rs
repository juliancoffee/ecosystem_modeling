use rand::prelude::SliceRandom;
use rand::Rng;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    Plant,
    VegeterianAnimal,
    PredatorAnimal,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Unit {
    kind: Kind,
    id: u32,
    quantity: f64,
}

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Water,
    Ground(Vec<Unit>),
}

impl Cell {
    fn print_stats(&self) {
        match self {
            Cell::Water => {}
            Cell::Ground(units) => {
                let mut unit_stats = Vec::new();
                for unit in units {
                    let code = match unit.kind {
                        Kind::Plant => "*",
                        Kind::VegeterianAnimal => "vegan",
                        Kind::PredatorAnimal => "pred",
                    };
                    unit_stats.push(format!(
                        "{}{}({:.2})",
                        code, unit.id, unit.quantity
                    ));
                }
                println!("{:?}", unit_stats);
            }
        }
    }
}

struct IdGen {
    counter: u32,
}

impl IdGen {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn next(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }

    fn total(&self) -> u32 {
        self.counter
    }
}

#[derive(Debug)]
struct Map {
    cells: Box<[[Cell; 10]; 10]>,
    generation: u32,
    units: u32,
}

impl Map {
    pub fn new() -> Self {
        // Utitlity closure for creating water cell
        let e = || Cell::Water;

        // Utility closure for creating ground cell
        let mut id_gen = IdGen::new();
        let mut g = || {
            // Utility variables for randomness
            let mut rng = rand::thread_rng();
            let kinds =
                &[Kind::Plant, Kind::VegeterianAnimal, Kind::PredatorAnimal];

            // Populate cell with one plant unit
            let mut units = vec![
                Unit {
                    id: *&mut id_gen.next(),
                    quantity: 50.0,
                    kind: Kind::Plant,
                },
                Unit {
                    id: *&mut id_gen.next(),
                    quantity: 10.0,
                    kind: Kind::VegeterianAnimal,
                },
            ];

            // Populate cell with random units
            let unit_limit = rng.gen_range(3..6);
            for _ in 0..unit_limit {
                let id = &mut id_gen.next();
                let kind = kinds.choose(&mut rng).unwrap();
                units.push(Unit {
                    id: *id,
                    quantity: 50.0,
                    kind: *kind,
                });
            }

            // Return cell
            Cell::Ground(units)
        };

        let cells = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 1, 1, 1, 1, 0, 1, 1, 0, 0],
            [0, 0, 1, 1, 1, 1, 1, 1, 0, 0],
            [0, 0, 1, 1, 1, 1, 1, 1, 0, 0],
            [0, 0, 0, 1, 1, 1, 1, 0, 0, 1],
            [0, 0, 1, 1, 1, 1, 1, 0, 1, 1],
            [0, 0, 0, 0, 1, 1, 1, 1, 1, 0],
            [0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let cells = cells
            .map(|line| line.map(|code| if code == 0 { e() } else { g() }));

        Self {
            generation: 0,
            cells: Box::new(cells),
            units: id_gen.total(),
        }
    }

    pub fn tick(&mut self) {
        self.generation += 1;

        self.feed_vegeterians();
        self.feed_predators();

        self.finalize();
    }

    fn feed_vegeterians(&mut self) {
        for line in self.cells.iter_mut() {
            for cell in line {
                let config = EvolutionConfig {
                    hunter_death: 21.0,
                    consuming_rate: 0.1,
                    resource_growth: 10.0,
                };
                hunt(cell, Kind::VegeterianAnimal, Kind::Plant, config);
            }
        }
    }

    fn feed_predators(&mut self) {
        for line in self.cells.iter_mut() {
            for cell in line {
                let config = EvolutionConfig {
                    hunter_death: 1.0,
                    consuming_rate: 0.25,
                    resource_growth: 12.0,
                };
                hunt(
                    cell,
                    Kind::PredatorAnimal,
                    Kind::VegeterianAnimal,
                    config,
                );
            }
        }
    }

    fn finalize(&mut self) {
        for line in self.cells.iter_mut() {
            for cell in line {
                match cell {
                    Cell::Water => {}
                    Cell::Ground(units) => {
                        units.retain(|unit| unit.quantity > 0.01)
                    }
                }
            }
        }
    }

    pub fn print_stats(&self) {
        for line in self.cells.iter() {
            for cell in line {
                cell.print_stats();
            }
            println!();
        }
    }
}

struct EvolutionConfig {
    hunter_death: f64,
    resource_growth: f64,
    consuming_rate: f64,
}

fn hunt(
    cell: &mut Cell,
    hunter_kind: Kind,
    target_kind: Kind,
    evolution: EvolutionConfig,
) {
    const TICK_RATE: f64 = 0.05;

    let EvolutionConfig {
        hunter_death,
        resource_growth,
        consuming_rate,
    } = evolution;
    match cell {
        Cell::Water => {}
        Cell::Ground(units) => {
            // Gather information about targets and hunters
            let mut targets = Vec::new();
            let mut hunters = Vec::new();
            for (idx, unit) in units.iter().enumerate() {
                if unit.kind == target_kind {
                    targets.push((idx, unit.quantity))
                } else if unit.kind == hunter_kind {
                    hunters.push((idx, unit.quantity));
                }
            }

            // Consume resource and reduce its quantity
            for (target_idx, resource) in &targets {
                let resource = *resource as f64;
                let mut consumed = 0.0;

                for (_hunter_idx, population) in &hunters {
                    let population = *population as f64;
                    consumed += resource * consuming_rate * population;
                }

                let dr = resource_growth - consumed;
                units[*target_idx].quantity += dr * TICK_RATE;
            }

            // Consume resource and feed hunters
            for (hunter_idx, population) in &hunters {
                let population = *population as f64;
                let mut consumed = 0.0;
                for (_target_idx, resource) in &targets {
                    let resource = *resource as f64;
                    consumed += resource * consuming_rate;
                }
                let dn = population * (consumed - hunter_death);
                units[*hunter_idx].quantity += dn * TICK_RATE;
            }
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = self
            .cells
            .clone()
            .map(|line| {
                line.map(|cell| if cell == Cell::Water { "-" } else { "^" })
                    .join(" ")
            })
            .join("\n");
        write!(f, "{}", content)
    }
}

fn main() {
    let mut map = Map::new();

    println!("{}", map);
    map.print_stats();
    for _ in 0..100 {
        map.tick();
        println!("=========================");
        map.print_stats();
    }
}
