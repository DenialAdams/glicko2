# glicko2
[![glicko2 crate](https://img.shields.io/crates/v/glicko2.svg)](https://crates.io/crates/glicko2)
[![glicko2 documentation](https://docs.rs/glicko2/badge.svg)](https://docs.rs/glicko2)

This crate implements the [glicko2 rating system](http://www.glicko.net/glicko/glicko2.pdf). It's a rating system appropriate for 1v1 games and is leveraged by many chess leagues.
## Usage

This example comes straight from the glicko2 rating pdf:

```rust
extern crate glicko2;

use glicko2::{GameResult, GlickoPlayer};

fn main() {
    let example_player = GlickoPlayer {
        rating: 1500.0,
        rating_deviation: 200.0,
    };
    let mut results = vec!();
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
    // Because we are passing in a GlickoPlayer instead of a Glicko2Player, we get a GlickoPlayer back.
    // This means that we lose the volatility field.
    // If you want to preserve volatility (i.e. use Glicko2), pass in a Glicko2Player instead.
    let new_player = glicko2::new_rating(example_player, &results, 0.5);
    println!("New rating: {} New rating deviation: {}", new_player.rating, new_player.rating_deviation);
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
