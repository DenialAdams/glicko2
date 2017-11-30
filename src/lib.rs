const CONVERGENCE_TOLERANCE: f64 = 0.000001;

/// Represents the rating of a player or team on the Glicko2 scale.
#[derive(Clone, Copy, Debug)]
pub struct Glicko2Rating {
    pub value: f64,
    pub deviation: f64,
    pub volatility: f64,
}

/// Represents the rating of a player or team on the Glicko (not Glicko2) scale.
///
/// Glicko2 rating numbers tend to be less friendly for humans,
/// so it's common to convert ratings to the Glicko scale before display.
#[derive(Clone, Copy, Debug)]
pub struct GlickoRating {
    pub value: f64,
    pub deviation: f64,
}

/// Represents a result (win, loss, or draw) over an opposing player or team.
///
/// Note well that only the opponent is stored in a `GameResult`.
/// The player that actually won, lost or drew respectively is not stored
/// in the game result, but instead is passed in to [`new_rating`](fn.new_rating.html).
#[derive(Clone, Copy, Debug)]
pub struct GameResult {
    // GLICKO2
    opponent_rating_value: f64,
    opponent_rating_deviation: f64,
    score: f64,
}

impl GameResult {
    /// Constructs a new game result representing a win over a player or team
    /// with rating `opponent_rating`.
    ///
    /// A Glicko2Rating or GlickoRating can be supplied for `opponent_rating`,
    /// and it will not affect the result of rating calculations
    /// as the volatility of opponents are not looked at for updating ratings.
    pub fn win<T: Into<Glicko2Rating>>(opponent_rating: T) -> GameResult {
        let opponent_glicko2: Glicko2Rating = opponent_rating.into();
        GameResult {
            opponent_rating_value: opponent_glicko2.value,
            opponent_rating_deviation: opponent_glicko2.deviation,
            score: 1.0,
        }
    }

    /// Constructs a new game result representing a loss to a player or team
    /// with rating `opponent_rating`.
    ///
    /// A Glicko2Rating or GlickoRating can be supplied for `opponent_rating`,
    /// and it will not affect the result of rating calculations
    /// as the volatility of opponents are not looked at for updating ratings.
    pub fn loss<T: Into<Glicko2Rating>>(opponent_rating: T) -> GameResult {
        let opponent_glicko2: Glicko2Rating = opponent_rating.into();
        GameResult {
            opponent_rating_value: opponent_glicko2.value,
            opponent_rating_deviation: opponent_glicko2.deviation,
            score: 0.0,
        }
    }

    /// Constructs a new game result representing a draw with a player or team
    /// with rating `opponent_rating`.
    ///
    /// A Glicko2Rating or GlickoRating can be supplied for `opponent_rating`,
    /// and it will not affect the result of rating calculations
    /// as the volatility of opponents are not looked at for updating ratings.
    pub fn draw<T: Into<Glicko2Rating>>(opponent_rating: T) -> GameResult {
        let opponent_glicko2: Glicko2Rating = opponent_rating.into();
        GameResult {
            opponent_rating_value: opponent_glicko2.value,
            opponent_rating_deviation: opponent_glicko2.deviation,
            score: 0.5,
        }
    }
}

impl From<GlickoRating> for Glicko2Rating {
    fn from(rating: GlickoRating) -> Glicko2Rating {
        Glicko2Rating {
            value: (rating.value - 1500.0) / 173.7178,
            deviation: rating.deviation / 173.7178,
            volatility: 0.06,
        }
    }
}

impl From<Glicko2Rating> for GlickoRating {
    fn from(rating: Glicko2Rating) -> GlickoRating {
        GlickoRating {
            value: rating.value * 173.7178 + 1500.0,
            deviation: rating.deviation * 173.7178,
        }
    }
}

impl Glicko2Rating {
    /// Constructs a `Glicko2Rating` using the defaults for a new (unrated) player or team.
    pub fn unrated() -> Glicko2Rating {
        Glicko2Rating::from(GlickoRating::unrated())
    }
}

impl GlickoRating {
    /// Constructs a `GlickoRating` using the defaults for a new (unrated) player or team.
    pub fn unrated() -> GlickoRating {
        GlickoRating {
            value: 1500.0,
            deviation: 350.0,
        }
    }
}

impl Default for Glicko2Rating {
    fn default() -> Glicko2Rating {
        Glicko2Rating::unrated()
    }
}

impl Default for GlickoRating {
    fn default() -> GlickoRating {
        GlickoRating::unrated()
    }
}

// The rest is best read with a copy of the glicko2 example PDF;
// I've tried to keep naming somewhat consistent
// http://www.glicko.net/glicko/glicko2.pdf
// One difference is that what is referred to in the pdf as 'player'
// I am referring to as a `rating`, and what is referred to as `rating`
// I am referring to as a `value`. I think that these changes make
// the API more clear, hopefully it's not too confusing.

fn g(rating_deviation: f64) -> f64 {
    use std::f64::consts::PI;
    let denom = 1.0 + ((3.0 * rating_deviation * rating_deviation) / (PI * PI));
    denom.sqrt().recip()
}

fn e(rating: f64, other_rating: f64, other_rating_deviation: f64) -> f64 {
    let base = -1.0 * g(other_rating_deviation) * (rating - other_rating);
    (1.0 + base.exp()).recip()
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

/// Calculates a new rating from an existing rating and a series of results.
///
/// Unlike `GameResult`s, which can be constructed with a `Glicko2Rating` or a`GlickoRating`,
/// `new_rating` requires a `Glicko2Rating`. This is because the volatility field present only in
/// `Glicko2Rating` affects the result of the calculation. Using a default volatility can be done,
/// but must be made explicit at the call site using the `Into<GlickoRating>` impl.
/// Similarly, converting the final result back to a `GlickoRating` and thus losing data is left to
/// the caller. Generally, converting back to a `GlickoRating` is only needed for display purposes.
///
/// `sys_constant` is best explained in the words of Mark Glickman himself:
/// > The system constant, τ, which constrains the change in volatility over time, needs to be
/// > set prior to application of the system. Reasonable choices are between 0.3 and 1.2,
/// > though the system should be tested to decide which value results in greatest predictive
/// > accuracy. Smaller values of τ prevent the volatility measures from changing by large
/// > amounts, which in turn prevent enormous changes in ratings based on very improbable
/// > results.
pub fn new_rating(
    prior_rating: Glicko2Rating,
    results: &[GameResult],
    sys_constant: f64,
) -> Glicko2Rating {
    if !results.is_empty() {
        let v: f64 = {
            results
                .iter()
                .fold(0.0, |acc, result| {
                    acc
                        + g(result.opponent_rating_deviation) * g(result.opponent_rating_deviation)
                            * e(
                                prior_rating.value,
                                result.opponent_rating_value,
                                result.opponent_rating_deviation,
                            )
                            * (1.0
                                - e(
                                    prior_rating.value,
                                    result.opponent_rating_value,
                                    result.opponent_rating_deviation,
                                ))
                })
                .recip()
        };
        let delta = {
            v * results.iter().fold(0.0, |acc, result| {
                acc
                    + g(result.opponent_rating_deviation)
                        * (result.score
                            - e(
                                prior_rating.value,
                                result.opponent_rating_value,
                                result.opponent_rating_deviation,
                            ))
            })
        };
        let new_volatility = {
            let mut a = (prior_rating.volatility * prior_rating.volatility).ln();
            let delta_squared = delta * delta;
            let rd_squared = prior_rating.deviation * prior_rating.deviation;
            let mut b = if delta_squared > rd_squared + v {
                delta_squared - rd_squared - v
            } else {
                let mut k = 1.0;
                while f(
                    a - k * sys_constant,
                    delta,
                    prior_rating.deviation,
                    v,
                    prior_rating.volatility,
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
                prior_rating.deviation,
                v,
                prior_rating.volatility,
                sys_constant,
            );
            let mut fb = f(
                b,
                delta,
                prior_rating.deviation,
                v,
                prior_rating.volatility,
                sys_constant,
            );
            while (b - a).abs() > CONVERGENCE_TOLERANCE {
                // a
                let c = a + ((a - b) * fa / (fb - fa));
                let fc = f(
                    c,
                    delta,
                    prior_rating.deviation,
                    v,
                    prior_rating.volatility,
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
        let new_pre_rd = ((prior_rating.deviation * prior_rating.deviation)
            + (new_volatility * new_volatility))
            .sqrt();
        let new_rd = {
            let subexpr_1 = (new_pre_rd * new_pre_rd).recip();
            let subexpr_2 = v.recip();
            (subexpr_1 + subexpr_2).sqrt().recip()
        };
        let new_rating = {
            prior_rating.value + ((new_rd * new_rd) * results.iter().fold(0.0, |acc, &result| {
                acc
                    + g(result.opponent_rating_deviation)
                        * (result.score
                            - e(
                                prior_rating.value,
                                result.opponent_rating_value,
                                result.opponent_rating_deviation,
                            ))
            }))
        };
        Glicko2Rating {
            value: new_rating,
            deviation: new_rd,
            volatility: new_volatility,
        }
    } else {
        let new_rd = ((prior_rating.deviation * prior_rating.deviation)
            + (prior_rating.volatility * prior_rating.volatility))
            .sqrt();
        Glicko2Rating {
            value: prior_rating.value,
            deviation: new_rd,
            volatility: prior_rating.volatility,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate approx;
    use self::approx::*;
    use super::*;

    #[test]
    fn test_rating_update() {
        let example_player_rating = Glicko2Rating::from(GlickoRating {
            value: 1500.0,
            deviation: 200.0,
        });
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
        let new_rating = new_rating(example_player_rating, &results, 0.5);
        assert!(
            Relative::new(&new_rating.value, &-0.2069)
                .epsilon(0.0001)
                .eq()
        );
        assert!(
            Relative::new(&new_rating.deviation, &0.8722)
                .epsilon(0.0001)
                .eq()
        );
        assert!(
            Relative::new(&new_rating.volatility, &0.05999)
                .epsilon(0.0001)
                .eq()
        );
    }

    #[test]
    fn test_glicko_glicko2_conversions() {
        let example_player = GlickoRating {
            value: 1500.0,
            deviation: 200.0,
        };
        let glicko2_rating = Glicko2Rating::from(example_player);
        assert!(
            Relative::new(&glicko2_rating.value, &0.0)
                .epsilon(0.0001)
                .eq()
        );
        assert!(
            Relative::new(&glicko2_rating.deviation, &1.1513)
                .epsilon(0.0001)
                .eq()
        );
        assert!(
            Relative::new(&glicko2_rating.volatility, &0.06)
                .epsilon(0.0001)
                .eq()
        );
        let glicko_rating = GlickoRating::from(glicko2_rating);
        assert!(
            Relative::new(&glicko_rating.value, &1500.0)
                .epsilon(0.0001)
                .eq()
        );
        assert!(
            Relative::new(&glicko_rating.deviation, &200.0)
                .epsilon(0.0001)
                .eq()
        );
    }
}
