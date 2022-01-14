use ecosystem_model::{Cell, EvolutionConfig, Map};
use serde_json::json;

fn experiment0(ticks: u32) -> Vec<Box<[[Cell; 5]; 2]>> {
    let mut generations = vec![];
    let config = EvolutionConfig {
        predator_death_rate: 2.8,
        predator_eating_rate: 0.075,
        vegeterian_death_rate: 2.8,
        vegeterian_eating_rate: 0.075,
        plant_growth_rate: 100.0,
    };

    let mut map = Map::experiment_flat(config);
    generations.push(map.cells.clone());
    for _ in 0..ticks {
        map.tick();
        generations.push(map.cells.clone());
    }
    generations
}

fn main() {
    let ticks = std::env::var("GENERATIONS")
        .ok()
        .and_then(|res| res.parse::<u32>().ok())
        .unwrap_or(100);
    let zero = experiment0(ticks);
    let deserialed = json!(zero);
    println!("{deserialed}");
}
