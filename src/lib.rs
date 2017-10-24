const CONVERGENCE_TOLERANCE: f64 = 0.000001;

/// Represents the rating of a player on the Glicko2 scale.
#[derive(Clone, Copy, Debug)]
pub struct Glicko2Player {
    pub rating: f64,
    pub rating_deviation: f64,
    pub volatility: f64,
}

/// Represents the rating of a player on the Glicko (not glicko2) scale.
/// Glicko2 rating numbers tend to be less friendly for humans,
/// so it's common to convert ratings to the Glicko scale before display.
#[derive(Clone, Copy, Debug)]
pub struct GlickoPlayer {
    pub rating: f64,
    pub rating_deviation: f64,
}

/// Represents a result (win, loss, or draw) over an opposing player.
/// Note well that only the opponent is stored in a `GameResult`.
/// The player that actually won, lost or drew respectively is not stored
/// in the game result, but instead is passed in to [new_rating](fn.new_rating.html).
#[derive(Clone, Copy, Debug)]
pub struct GameResult {
    // GLICKO2
    opponent_rating: f64,
    opponent_rating_deviation: f64,
    score: f64,
}

impl GameResult {
    /// Constructs a new game result representing a win over `player`.
    pub fn win<T: Into<Glicko2Player>>(player: T) -> GameResult {
        let player: Glicko2Player = player.into();
        GameResult {
            opponent_rating: player.rating,
            opponent_rating_deviation: player.rating_deviation,
            score: 1.0,
        }
    }

    /// Constructs a new game result representing a loss to `player`.
    pub fn loss<T: Into<Glicko2Player>>(player: T) -> GameResult {
        let player: Glicko2Player = player.into();
        GameResult {
            opponent_rating: player.rating,
            opponent_rating_deviation: player.rating_deviation,
            score: 0.0,
        }
    }

    /// Constructs a new game result representing a draw with `player`.
    pub fn draw<T: Into<Glicko2Player>>(player: T) -> GameResult {
        let player: Glicko2Player = player.into();
        GameResult {
            opponent_rating: player.rating,
            opponent_rating_deviation: player.rating_deviation,
            score: 0.5,
        }
    }
}

impl From<GlickoPlayer> for Glicko2Player {
    fn from(player: GlickoPlayer) -> Glicko2Player {
        Glicko2Player {
            rating: (player.rating - 1500.0) / 173.7178,
            rating_deviation: player.rating_deviation / 173.7178,
            volatility: 0.06,
        }
    }
}

impl From<Glicko2Player> for GlickoPlayer {
    fn from(player: Glicko2Player) -> GlickoPlayer {
        GlickoPlayer {
            rating: player.rating * 173.7178 + 1500.0,
            rating_deviation: player.rating_deviation * 173.7178,
        }
    }
}

impl Glicko2Player {
    /// Constructs a `Glicko2Player` using the defaults for a new (unrated) player.
    pub fn unrated() -> Glicko2Player {
        Glicko2Player::from(GlickoPlayer::unrated())
    }
}

impl GlickoPlayer {
    /// Constructs a `GlickoPlayer` using the defaults for a new (unrated) player.
    pub fn unrated() -> GlickoPlayer {
        GlickoPlayer {
            rating: 1500.0,
            rating_deviation: 350.0,
        }
    }
}

// The rest is best read with a copy of the glicko2 example PDF;
// I've tried to keep naming somewhat consistent
// http://www.glicko.net/glicko/glicko2.pdf

fn g(rating_deviation: f64) -> f64 {
    use std::f64::consts::PI;
    let denom = 1.0 + ((3.0 * rating_deviation * rating_deviation) / (PI * PI));
    1.0 / denom.sqrt()
}

fn e(rating: f64, other_rating: f64, other_rating_deviation: f64) -> f64 {
    let base = -1.0 * g(other_rating_deviation) * (rating - other_rating);
    1.0 / (1.0 + base.exp())
}

fn f(x: f64, delta: f64, rating_deviation: f64, v: f64, volatility: f64, sys_constant: f64) -> f64 {
    let fraction_one = {
        let numer =
            x.exp() * ((delta * delta) - (rating_deviation * rating_deviation) - v - x.exp());
        let denom = 2.0 * (rating_deviation * rating_deviation + v + x.exp())
            * (rating_deviation * rating_deviation + v + x.exp());
        numer / denom
    };
    let fraction_two = {
        let numer = x - (volatility * volatility).ln();
        let denom = sys_constant * sys_constant;
        numer / denom
    };
    fraction_one - fraction_two
}

/// Generate a new Rating from an existing rating and a series of results.
///
/// `sys_constant` is best explained in the words of Mark Glickman himself:
/// > The system constant, τ, which constrains the change in volatility over time, needs to be
/// > set prior to application of the system. Reasonable choices are between 0.3 and 1.2,
/// > though the system should be tested to decide which value results in greatest predictive
/// > accuracy. Smaller values of τ prevent the volatility measures from changing by large
/// > amounts, which in turn prevent enormous changes in ratings based on very improbable
/// > results.
pub fn new_rating<T: Into<Glicko2Player> + From<Glicko2Player>>(
    player: T,
    results: &[GameResult],
    sys_constant: f64,
) -> T {
    let player: Glicko2Player = player.into();
    if !results.is_empty() {
        let v: f64 = {
            let mut sum = 0.0;
            for result in results {
                let mut p =
                    g(result.opponent_rating_deviation) * g(result.opponent_rating_deviation);
                p *= e(
                    player.rating,
                    result.opponent_rating,
                    result.opponent_rating_deviation,
                );
                p *= 1.0
                    - e(
                        player.rating,
                        result.opponent_rating,
                        result.opponent_rating_deviation,
                    );
                sum += p;
            }
            1.0 / sum
        };
        let delta = {
            let mut sum = 0.0;
            for result in results {
                let mut p = g(result.opponent_rating_deviation);
                p *= result.score
                    - e(
                        player.rating,
                        result.opponent_rating,
                        result.opponent_rating_deviation,
                    );
                sum += p;
            }
            v * sum
        };
        let new_volatility = {
            let mut a = (player.volatility * player.volatility).ln();
            let delta_squared = delta * delta;
            let rd_squared = player.rating_deviation * player.rating_deviation;
            let mut b = if delta_squared > rd_squared + v {
                delta_squared - rd_squared - v
            } else {
                let mut k = 1.0;
                while f(
                    a - k * sys_constant,
                    delta,
                    player.rating_deviation,
                    v,
                    player.volatility,
                    sys_constant,
                ) < 0.0
                {
                    k += 1.0;
                }
                a - k * sys_constant
            };
            let mut fa = f(
                a,
                delta,
                player.rating_deviation,
                v,
                player.volatility,
                sys_constant,
            );
            let mut fb = f(
                b,
                delta,
                player.rating_deviation,
                v,
                player.volatility,
                sys_constant,
            );
            while (b - a).abs() > CONVERGENCE_TOLERANCE {
                // a
                let c = a + ((a - b) * fa / (fb - fa));
                let fc = f(
                    c,
                    delta,
                    player.rating_deviation,
                    v,
                    player.volatility,
                    sys_constant,
                );
                // b
                if fc * fb < 0.0 {
                    a = b;
                    fa = fb;
                } else {
                    fa /= 2.0;
                }
                // c
                b = c;
                fb = fc;
                // d (while loop)
            }
            (a / 2.0).exp()
        };
        let new_pre_rd = ((player.rating_deviation * player.rating_deviation)
            + (new_volatility * new_volatility))
            .sqrt();
        let new_rd = {
            let subexpr_1 = 1.0 / (new_pre_rd * new_pre_rd);
            let subexpr_2 = 1.0 / v;
            1.0 / (subexpr_1 + subexpr_2).sqrt()
        };
        let new_rating = {
            let mut sum = 0.0;
            for result in results {
                let mut p = g(result.opponent_rating_deviation);
                p *= result.score
                    - e(
                        player.rating,
                        result.opponent_rating,
                        result.opponent_rating_deviation,
                    );
                sum += p;
            }
            player.rating + ((new_rd * new_rd) * sum)
        };
        Glicko2Player {
            rating: new_rating,
            rating_deviation: new_rd,
            volatility: new_volatility,
        }.into()
    } else {
        let new_rd = ((player.rating_deviation * player.rating_deviation)
            + (player.volatility * player.volatility))
            .sqrt();
        Glicko2Player {
            rating: player.rating,
            rating_deviation: new_rd,
            volatility: player.volatility,
        }.into()
    }
}

#[cfg(test)]
mod tests {
    extern crate approx;
    use self::approx::*;
    use super::*;

    #[test]
    fn test_rating_update() {
        let example_player = Glicko2Player::from(GlickoPlayer {
            rating: 1500.0,
            rating_deviation: 200.0,
        });
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
        let new_player = new_rating(example_player, &results, 0.5);
        assert!(
            Relative::new(&new_player.rating, &-0.2069)
                .epsilon(0.0001)
                .eq()
        );
        assert!(
            Relative::new(&new_player.rating_deviation, &0.8722)
                .epsilon(0.0001)
                .eq()
        );
        assert!(
            Relative::new(&new_player.volatility, &0.05999)
                .epsilon(0.0001)
                .eq()
        );
    }

    #[test]
    fn test_glicko_glicko2_conversions() {
        let example_player = GlickoPlayer {
            rating: 1500.0,
            rating_deviation: 200.0,
        };
        let glicko2_player = Glicko2Player::from(example_player);
        assert!(
            Relative::new(&glicko2_player.rating, &0.0)
                .epsilon(0.0001)
                .eq()
        );
        assert!(
            Relative::new(&glicko2_player.rating_deviation, &1.1513)
                .epsilon(0.0001)
                .eq()
        );
        assert!(
            Relative::new(&glicko2_player.volatility, &0.06)
                .epsilon(0.0001)
                .eq()
        );
        let glicko_player = GlickoPlayer::from(glicko2_player);
        assert!(
            Relative::new(&glicko_player.rating, &1500.0)
                .epsilon(0.0001)
                .eq()
        );
        assert!(
            Relative::new(&glicko_player.rating_deviation, &200.0)
                .epsilon(0.0001)
                .eq()
        );
    }
}
