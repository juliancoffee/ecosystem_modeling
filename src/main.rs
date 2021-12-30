#[derive(Clone, Copy, Debug)]
enum Unit {
    Mineral,
    Plant,
    Animal,
}

#[derive(Clone, Debug)]
enum Cell {
    Water,
    Ground(Vec<(Unit, i32)>),
}

impl Default for Cell {
    fn default() -> Self {
        Self::Water
    }
}

#[derive(Debug, Default)]
struct Map {
    cells: Box<[[Cell; 10]; 10]>,
}

impl Map {
    fn new() -> Self {
        let e = || Cell::default();
        Self {
            cells: Box::new([
                [e(), e(), e(), e(), e(), e(), e(), e(), e(), e()],
                [e(), e(), e(), e(), e(), e(), e(), e(), e(), e()],
                [e(), e(), e(), e(), e(), e(), e(), e(), e(), e()],
                [e(), e(), e(), e(), e(), e(), e(), e(), e(), e()],
                [e(), e(), e(), e(), e(), e(), e(), e(), e(), e()],
                [e(), e(), e(), e(), e(), e(), e(), e(), e(), e()],
                [e(), e(), e(), e(), e(), e(), e(), e(), e(), e()],
                [e(), e(), e(), e(), e(), e(), e(), e(), e(), e()],
                [e(), e(), e(), e(), e(), e(), e(), e(), e(), e()],
                [e(), e(), e(), e(), e(), e(), e(), e(), e(), e()],
            ]),
        }
    }
}

fn main() {
    let map = Map::default();
    println!("{:#?}", map);
}
