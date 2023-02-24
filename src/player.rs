use crate::{
    tile::{Dragon, Simple, Tile, Wind},
    yaku::Yaku,
};

#[derive(Clone)]
pub struct Player {
    pub seat: Wind,
    pub hand: Vec<Tile>,
    pub dealt_in: Vec<Tile>,
    pub discarded: Vec<Tile>,
    pub kan: Vec<Tile>,
    pub riichi: bool,
}

impl Player {
    pub fn evaluate_yakus(self, prevalent_wind: Wind) -> Vec<Yaku> {
        let mut yakus = vec![];

        let mut hand = self.hand.clone();
        hand.sort();

        let mut dealt_in = self.dealt_in.clone();
        dealt_in.sort();

        let mut all_tiles_filtered = [hand.clone(), dealt_in.clone()].concat();
        all_tiles_filtered.sort();

        #[derive(Clone)]
        struct HandConfiguration {
            triplets: Vec<Vec<Tile>>,
            pair: Vec<Tile>,
        }

        // permutation of possible configurations for the player's hand. there
        // are 5 possible configurations in total considering they're always 4
        // triplets and 1 pair. invalid configurations are discarded.
        let mut hand_permutations: Vec<HandConfiguration> = vec![];
        {
            for index in 0..12 {
                let mut permutation = all_tiles_filtered.clone();
                let pair =
                    [permutation.remove(index + 1), permutation.remove(index)];

                if pair[0] != pair[1] {
                    continue;
                }

                let configuration = HandConfiguration {
                    triplets: permutation
                        .chunks(3)
                        .map(|x| x.to_vec())
                        .collect(),
                    pair: pair.to_vec(),
                };

                let is_valid_triplet = |tiles: Vec<Tile>| -> bool {
                    if tiles[0] == tiles[1] && tiles[0] == tiles[2] {
                        return true;
                    }
                    return match tiles[0] {
                        Tile::Simple(Simple::Man(number1)) => match tiles[1] {
                            Tile::Simple(Simple::Man(number2)) => {
                                match tiles[2] {
                                    Tile::Simple(Simple::Man(number3)) => {
                                        number1 == number2 - 1
                                            && number2 == number3 - 1
                                    }
                                    _ => false,
                                }
                            }
                            _ => false,
                        },
                        Tile::Simple(Simple::Pin(number1)) => match tiles[1] {
                            Tile::Simple(Simple::Pin(number2)) => {
                                match tiles[2] {
                                    Tile::Simple(Simple::Pin(number3)) => {
                                        number1 == number2 - 1
                                            && number2 == number3 - 1
                                    }
                                    _ => false,
                                }
                            }
                            _ => false,
                        },
                        Tile::Simple(Simple::Sou(number1)) => match tiles[1] {
                            Tile::Simple(Simple::Sou(number2)) => {
                                match tiles[2] {
                                    Tile::Simple(Simple::Sou(number3)) => {
                                        number1 == number2 - 1
                                            && number2 == number3 - 1
                                    }
                                    _ => false,
                                }
                            }
                            _ => false,
                        },
                        _ => false,
                    };
                };

                let mut is_valid_conf = true;
                for triplet in configuration.triplets.clone() {
                    if !is_valid_triplet(triplet) {
                        is_valid_conf = false;
                        break;
                    }
                }

                if !is_valid_conf {
                    continue;
                }

                hand_permutations.push(configuration);
            }
        }

        {
            // checks for seat wind and prevalent wind.
            let wind_tiles: Vec<Tile> = hand
                .iter()
                .cloned()
                .filter(|x| match x {
                    Tile::Wind(_) => true,
                    _ => false,
                })
                .collect();

            if wind_tiles.len() >= 3 {
                for wind_type in
                    [Wind::East, Wind::North, Wind::South, Wind::West]
                {
                    let count = wind_tiles
                        .iter()
                        .filter(|t| **t == Tile::Wind(wind_type))
                        .count();

                    if count >= 3 {
                        if wind_type == prevalent_wind {
                            yakus.push(Yaku::PrevalentWind(wind_type));
                        } else if wind_type == self.seat {
                            yakus.push(Yaku::SeatWind(wind_type));
                        }
                    }
                }
            }
        }

        {
            // checks for dragon.
            let dragon_tiles: Vec<Tile> = hand
                .iter()
                .cloned()
                .filter(|x| match x {
                    Tile::Dragon(_) => true,
                    _ => false,
                })
                .collect();

            if dragon_tiles.len() >= 3 {
                for dragon_type in [Dragon::Green, Dragon::Red, Dragon::White] {
                    let count = dragon_tiles
                        .iter()
                        .filter(|t| **t == Tile::Dragon(dragon_type))
                        .count();

                    if count >= 3 {
                        yakus.push(Yaku::Dragons(dragon_type));
                    }
                }
            }
        }

        {
            // checks for seven pairs
            let mut unique_check = Tile::None;
            // needs to be the original hand because we removed Kans (and Kan
            // invalidates this yaku).
            for pair in self.hand.chunks(2).into_iter() {
                if pair[0] == pair[1] && pair[0] != unique_check {
                    unique_check = pair[0];
                } else {
                    unique_check = Tile::None;
                    break;
                }
            }
            if unique_check != Tile::None {
                yakus.push(Yaku::SevenPairs);
            }
        }

        if self.riichi && !yakus.is_empty() {
            yakus.push(Yaku::Riichi);
        }

        yakus
    }

    pub fn is_menzenchin(&self) -> bool {
        return self.dealt_in.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        player::Player,
        tile::{Dragon, Simple, Tile, Wind},
        yaku::Yaku,
    };

    #[test]
    fn is_menzenchin() {
        let mut player = Player {
            seat: Wind::East,
            hand: vec![
                Tile::Dragon(Dragon::Red),
                Tile::Dragon(Dragon::Red),
                Tile::Dragon(Dragon::Red),
                Tile::Simple(Simple::Man(1)),
                Tile::Simple(Simple::Man(2)),
                Tile::Simple(Simple::Man(3)),
                Tile::Simple(Simple::Pin(1)),
                Tile::Simple(Simple::Pin(2)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Dragon(Dragon::Green),
                Tile::Dragon(Dragon::Green),
                Tile::Dragon(Dragon::Green),
            ],
            dealt_in: vec![],
            discarded: vec![],
            kan: vec![],
            riichi: false,
        };

        assert!(player.is_menzenchin());

        player.hand.truncate(11);
        for _ in 0..3 {
            player.dealt_in.push(Tile::Simple(Simple::Man(1)));
        }

        assert!(!player.is_menzenchin());
    }

    #[test]
    fn dragon() {
        let mut player = Player {
            seat: Wind::East,
            hand: vec![
                Tile::Wind(Wind::East),
                Tile::Wind(Wind::East),
                Tile::Wind(Wind::East),
                Tile::Simple(Simple::Man(1)),
                Tile::Simple(Simple::Man(2)),
                Tile::Simple(Simple::Man(3)),
                Tile::Simple(Simple::Pin(1)),
                Tile::Simple(Simple::Pin(2)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Dragon(Dragon::Green),
                Tile::Dragon(Dragon::Green),
                Tile::Dragon(Dragon::Green),
            ],
            dealt_in: vec![],
            discarded: vec![],
            kan: vec![],
            riichi: false,
        };

        assert!(player
            .clone()
            .evaluate_yakus(Wind::East)
            .contains(&Yaku::Dragons(Dragon::Green)));

        player.hand.pop();
        player.hand.push(Tile::Simple(Simple::Man(1)));

        assert!(!player
            .clone()
            .evaluate_yakus(Wind::East)
            .contains(&Yaku::Dragons(Dragon::Green)));
    }

    #[test]
    fn seat_wind() {
        let mut player = Player {
            seat: Wind::West,
            hand: vec![
                Tile::Simple(Simple::Man(1)),
                Tile::Simple(Simple::Man(2)),
                Tile::Simple(Simple::Man(3)),
                Tile::Simple(Simple::Pin(1)),
                Tile::Simple(Simple::Pin(2)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Dragon(Dragon::Green),
                Tile::Dragon(Dragon::Green),
                Tile::Dragon(Dragon::Green),
                Tile::Wind(Wind::West),
                Tile::Wind(Wind::West),
                Tile::Wind(Wind::West),
            ],
            dealt_in: vec![],
            discarded: vec![],
            kan: vec![],
            riichi: false,
        };

        assert!(player
            .clone()
            .evaluate_yakus(Wind::East)
            .contains(&Yaku::SeatWind(Wind::West)));

        player.hand.pop();
        player.hand.push(Tile::Simple(Simple::Man(1)));

        assert!(!player
            .clone()
            .evaluate_yakus(Wind::East)
            .contains(&Yaku::SeatWind(Wind::West)));
    }

    #[test]
    fn prevalent_wind() {
        let mut player = Player {
            seat: Wind::West,
            hand: vec![
                Tile::Simple(Simple::Man(1)),
                Tile::Simple(Simple::Man(2)),
                Tile::Simple(Simple::Man(3)),
                Tile::Simple(Simple::Pin(1)),
                Tile::Simple(Simple::Pin(2)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Dragon(Dragon::Green),
                Tile::Dragon(Dragon::Green),
                Tile::Dragon(Dragon::Green),
                Tile::Wind(Wind::East),
                Tile::Wind(Wind::East),
                Tile::Wind(Wind::East),
            ],
            dealt_in: vec![],
            discarded: vec![],
            kan: vec![],
            riichi: false,
        };

        assert!(player
            .clone()
            .evaluate_yakus(Wind::East)
            .contains(&Yaku::PrevalentWind(Wind::East)));

        player.hand.pop();
        player.hand.push(Tile::Simple(Simple::Man(1)));

        assert!(!player
            .clone()
            .evaluate_yakus(Wind::East)
            .contains(&Yaku::PrevalentWind(Wind::East)));
    }

    #[test]
    fn riichi() {
        let mut player = Player {
            seat: Wind::East,
            hand: vec![
                Tile::Wind(Wind::East),
                Tile::Wind(Wind::East),
                Tile::Wind(Wind::East),
                Tile::Dragon(Dragon::Red),
                Tile::Dragon(Dragon::Green),
                Tile::Dragon(Dragon::White),
                Tile::Simple(Simple::Man(1)),
                Tile::Simple(Simple::Man(2)),
                Tile::Simple(Simple::Man(3)),
                Tile::Simple(Simple::Pin(1)),
                Tile::Simple(Simple::Pin(2)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Simple(Simple::Pin(3)),
            ],
            dealt_in: vec![],
            discarded: vec![],
            kan: vec![],
            riichi: true,
        };

        assert!(player
            .clone()
            .evaluate_yakus(Wind::East)
            .contains(&Yaku::Riichi));
        player.riichi = false;
        assert!(!player.evaluate_yakus(Wind::East).contains(&Yaku::Riichi));
    }

    #[test]
    fn seven_pairs() {
        let mut player = Player {
            seat: Wind::East,
            hand: vec![
                Tile::Wind(Wind::East),
                Tile::Wind(Wind::East),
                Tile::Dragon(Dragon::Red),
                Tile::Dragon(Dragon::Red),
                Tile::Dragon(Dragon::Green),
                Tile::Dragon(Dragon::Green),
                Tile::Simple(Simple::Man(1)),
                Tile::Simple(Simple::Man(1)),
                Tile::Simple(Simple::Pin(1)),
                Tile::Simple(Simple::Pin(1)),
                Tile::Simple(Simple::Pin(2)),
                Tile::Simple(Simple::Pin(2)),
                Tile::Simple(Simple::Pin(3)),
                Tile::Simple(Simple::Pin(3)),
            ],
            dealt_in: vec![],
            discarded: vec![],
            kan: vec![],
            riichi: true,
        };

        assert!(player
            .clone()
            .evaluate_yakus(Wind::East)
            .contains(&Yaku::SevenPairs));

        player.hand.pop();
        player.hand.push(Tile::Simple(Simple::Man(1)));

        assert!(!player
            .clone()
            .evaluate_yakus(Wind::East)
            .contains(&Yaku::SevenPairs));
    }
}
