#[derive(Clone, Copy, Debug)]
enum Kind {
    Plant,
    Animal,
}

#[derive(Clone, Copy, Debug)]
struct Unit {
    kind: Kind,
    id: u32,
    quantity: u32,
}

#[derive(Clone, Debug)]
enum Cell {
    Water,
    Ground(Vec<Unit>),
}

#[derive(Debug)]
struct Map {
    cells: Box<[[Cell; 10]; 10]>,
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
}

impl Map {
    fn new() -> Self {
        // empty cell
        let e = || Cell::Water;
        // ground cell
        let mut id_gen = IdGen::new();
        let mut g = || {
            let mut units = Vec::new();
            for _ in 0..5 {
                let id = &mut id_gen.next();
                units.push(Unit {
                    id: *id,
                    quantity: 50,
                    kind: Kind::Plant,
                });
            }
            Cell::Ground(units)
        };

        Self {
            cells: Box::new([
                [e(), e(), e(), e(), e(), e(), e(), e(), e(), e()],
                [e(), g(), e(), e(), e(), e(), e(), g(), e(), e()],
                [e(), g(), g(), g(), g(), e(), g(), g(), e(), e()],
                [e(), e(), g(), g(), g(), g(), g(), g(), e(), e()],
                [e(), e(), g(), g(), g(), g(), g(), g(), e(), e()],
                [e(), e(), e(), g(), g(), g(), g(), e(), e(), g()],
                [e(), e(), g(), g(), g(), g(), g(), e(), g(), g()],
                [e(), e(), e(), e(), g(), g(), g(), g(), g(), e()],
                [e(), e(), e(), e(), g(), g(), g(), g(), e(), e()],
                [e(), e(), e(), e(), e(), e(), e(), e(), e(), e()],
            ]),
        }
    }

    fn tick(&mut self) {}
}

fn main() {
    let mut map = Map::new();

    map.tick();
    println!("{:#?}", map);
}
