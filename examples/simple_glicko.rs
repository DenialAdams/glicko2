extern crate glicko2;

use glicko2::{GameResult, GlickoPlayer};

fn main() {
    let example_player = GlickoPlayer {
        rating: 1500.0,
        rating_deviation: 200.0,
    };
    let mut results = vec![];
    results.push(GameResult::win(GlickoPlayer {
        rating: 1400.0,
        rating_deviation: 30.0,
    }));
    results.push(GameResult::loss(GlickoPlayer {
        rating: 1550.0,
        rating_deviation: 100.0,
    }));
    results.push(GameResult::loss(GlickoPlayer {
        rating: 1700.0,
        rating_deviation: 300.0,
    }));
    // Because we pass in a GlickoPlayer instead of a Glicko2Player, we get a GlickoPlayer back.
    // This means that we lose the volatility field.
    // If you want to preserve volatility (i.e. use Glicko2), pass in a Glicko2Player instead.
    let new_player = glicko2::new_rating(example_player, &results, 0.5);
    println!(
        "New rating: {} New rating deviation: {}",
        new_player.rating,
        new_player.rating_deviation
    );
}
