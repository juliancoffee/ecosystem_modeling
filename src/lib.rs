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

struct CellData {
    plants: u32,
    vegeterians: u32,
    predators: u32,
    unit_size: f64,
}

impl Default for CellData {
    fn default() -> Self {
        Self {
            plants: 0,
            vegeterians: 0,
            predators: 0,
            unit_size: 50.0,
        }
    }
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
                    let id = unit.id;
                    let quantity = unit.quantity;
                    unit_stats.push(format!("{code}{id}({quantity:.2})",));
                }
                println!("{:?}", unit_stats);
            }
        }
    }

    #[allow(dead_code)]
    fn random_cell(id_gen: &mut IdGen) -> Self {
        // Utility variables for randomness
        let mut rng = rand::thread_rng();
        let plants = rng.gen_range(1..3);
        let vegeterians = rng.gen_range(0..2);
        let predators = rng.gen_range(0..2);
        let request = CellData {
            plants,
            vegeterians,
            predators,
            unit_size: 50.0,
        };

        Self::from_data(id_gen, request)
    }

    fn from_data(id_gen: &mut IdGen, request: CellData) -> Self {
        let mut units = Vec::new();
        let CellData {
            plants,
            vegeterians,
            predators,
            unit_size,
        } = request;

        for _ in 0..plants {
            units.push(Unit {
                id: id_gen.next(),
                quantity: unit_size,
                kind: Kind::Plant,
            });
        }
        for _ in 0..vegeterians {
            units.push(Unit {
                id: id_gen.next(),
                quantity: unit_size,
                kind: Kind::VegeterianAnimal,
            });
        }
        for _ in 0..predators {
            units.push(Unit {
                id: id_gen.next(),
                quantity: unit_size,
                kind: Kind::PredatorAnimal,
            });
        }

        Cell::Ground(units)
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
pub struct Map {
    cells: Box<[[Cell; 10]; 10]>,
    generation: u32,
    #[allow(dead_code)]
    units: u32,
    config: EvolutionConfig,
}

#[derive(Debug)]
pub struct EvolutionConfig {
    pub predator_death_rate: f64,
    pub predator_eating_rate: f64,
    pub vegeterian_death_rate: f64,
    pub vegeterian_eating_rate: f64,
    pub plant_growth_rate: f64,
}

impl Map {
    pub fn experiment_flat(config: EvolutionConfig) -> Self {
        let mut id_gen = IdGen::new();

        let mut cells = [[(); 10]; 10].map(|line| line.map(|_| Cell::Water));
        cells[0][0] = Cell::from_data(
            &mut id_gen,
            CellData {
                plants: 3,
                ..CellData::default()
            },
        );
        cells[0][1] = Cell::from_data(
            &mut id_gen,
            CellData {
                plants: 3,
                vegeterians: 1,
                ..CellData::default()
            },
        );
        cells[0][2] = Cell::from_data(
            &mut id_gen,
            CellData {
                plants: 1,
                vegeterians: 4,
                ..CellData::default()
            },
        );
        cells[0][3] = Cell::from_data(
            &mut id_gen,
            CellData {
                plants: 1,
                vegeterians: 5,
                ..CellData::default()
            },
        );
        cells[0][4] = Cell::from_data(
            &mut id_gen,
            CellData {
                plants: 0,
                vegeterians: 1,
                unit_size: 50.0,
                ..CellData::default()
            },
        );
        cells[1][0] = Cell::from_data(
            &mut id_gen,
            CellData {
                plants: 1,
                vegeterians: 1,
                predators: 1,
                ..CellData::default()
            },
        );
        cells[1][1] = Cell::from_data(
            &mut id_gen,
            CellData {
                plants: 1,
                vegeterians: 1,
                predators: 3,
                ..CellData::default()
            },
        );
        cells[1][2] = Cell::from_data(
            &mut id_gen,
            CellData {
                plants: 1,
                vegeterians: 1,
                predators: 5,
                ..CellData::default()
            },
        );
        cells[1][3] = Cell::from_data(
            &mut id_gen,
            CellData {
                plants: 0,
                vegeterians: 0,
                predators: 1,
                unit_size: 50.0,
                ..CellData::default()
            },
        );

        Self {
            generation: 0,
            cells: Box::new(cells),
            units: id_gen.total(),
            config,
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
                let config = HuntConfig {
                    hunter_death: self.config.vegeterian_death_rate,
                    consuming_rate: self.config.vegeterian_eating_rate,
                    resource_growth: self.config.plant_growth_rate,
                };
                hunt(cell, Kind::VegeterianAnimal, Kind::Plant, config);
            }
        }
    }

    fn feed_predators(&mut self) {
        for line in self.cells.iter_mut() {
            for cell in line {
                let config = HuntConfig {
                    hunter_death: self.config.predator_death_rate,
                    consuming_rate: self.config.predator_eating_rate,
                    resource_growth: 0.0,
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

struct HuntConfig {
    hunter_death: f64,
    resource_growth: f64,
    consuming_rate: f64,
}

fn hunt(
    cell: &mut Cell,
    hunter_kind: Kind,
    target_kind: Kind,
    evolution: HuntConfig,
) {
    const TICK_RATE: f64 = 0.075;

    let HuntConfig {
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
