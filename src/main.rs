use ecosystem_model::{EvolutionConfig, Map};

fn main() {
    let config = EvolutionConfig {
        predator_death_rate: 2.8,
        predator_eating_rate: 0.075,
        vegeterian_death_rate: 2.8,
        vegeterian_eating_rate: 0.075,
        plant_growth_rate: 100.0,
    };
    let mut map = Map::experiment_flat(config);

    println!("{}", map);
    let ticks = std::env::var("GENERATIONS")
        .ok()
        .and_then(|res| res.parse::<u32>().ok())
        .unwrap_or(100);

    map.print_stats();
    for _ in 0..ticks {
        map.tick();
        println!("=========================");
        map.print_stats();
    }
}
