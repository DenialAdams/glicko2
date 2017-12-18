# glicko2
[![glicko2 crate](https://img.shields.io/crates/v/glicko2.svg)](https://crates.io/crates/glicko2)
[![glicko2 documentation](https://docs.rs/glicko2/badge.svg)](https://docs.rs/glicko2)

This crate implements the [glicko2 rating system](http://www.glicko.net/glicko/glicko2.pdf). It's a rating system appropriate for rating a team or player and is leveraged by many chess leagues.
## Usage

This example comes straight from the glicko2 rating pdf:

```rust
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

```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
