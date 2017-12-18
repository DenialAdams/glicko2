extern crate glicko2;

use glicko2::{GameResult, GlickoRating};

fn main() {
    let example_rating = GlickoRating {
        value: 1500.0,
        deviation: 200.0,
    };
    let mut results = vec![];
    results.push(GameResult::win(GlickoRating {
        value: 1400.0,
        deviation: 30.0,
    }));
    results.push(GameResult::loss(GlickoRating {
        value: 1550.0,
        deviation: 100.0,
    }));
    results.push(GameResult::loss(GlickoRating {
        value: 1700.0,
        deviation: 300.0,
    }));
    // We are converting the result of new_rating to a GlickoRating immediately, throwing away the
    // benefits of Glicko2 over Glicko for the sake of matching the example in the glicko2 pdf.
    // In a real application, you'd likely want to save the Glicko2Rating and convert to
    // GlickoRating for display purposes only.
    let new_rating: GlickoRating = glicko2::new_rating(example_rating.into(), &results, 0.5).into();
    println!(
        "New rating value: {} New rating deviation: {}",
        new_rating.value,
        new_rating.deviation
    );
}
