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
    // We are converting the result of new_rating to a GlickoPlayer immediately, throwing away the
    // benefits of Glicko2 over Glicko for the sake of matching the example in the glicko2 pdf.
    // In a real application, you'd likely want to save the Glicko2Player and convert to
    // GlickoPlayer for display purposes only.
    let new_player: GlickoPlayer = glicko2::new_rating(example_player, &results, 0.5).into();
    println!(
        "New rating: {} New rating deviation: {}",
        new_player.rating,
        new_player.rating_deviation
    );
}
