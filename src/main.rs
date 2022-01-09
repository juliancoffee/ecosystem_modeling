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
    quantity: u32,
}

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Water,
    Ground(Vec<Unit>),
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
        // empty cell
        let e = || Cell::Water;
        // ground cell
        let mut id_gen = IdGen::new();
        let mut g = || {
            let mut rng = rand::thread_rng();
            let kinds = &[Kind::Plant, Kind::VegeterianAnimal, Kind::PredatorAnimal];
            let mut units = Vec::new();
            let unit_limit = rng.gen_range(3..6);
            for _ in 0..unit_limit {
                let id = &mut id_gen.next();
                let kind = kinds.choose(&mut rng).unwrap();
                units.push(Unit {
                    id: *id,
                    quantity: 50,
                    kind: *kind,
                });
            }
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

        let cells = cells.map(|line| line.map(|code| if code == 0 { e() } else { g() }));

        Self {
            generation: 0,
            cells: Box::new(cells),
            units: id_gen.total(),
        }
    }

    pub fn tick(&mut self) {
        self.generation += 1;
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
    println!("{:#?}", map);

    map.tick();
}
