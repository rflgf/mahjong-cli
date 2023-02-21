use crate::{tile::{Wind, Tile}, yaku::Yaku};

pub struct Player {
    pub seat: Wind,
    pub hand: Vec<Tile>,
    pub dealt_in: Vec<Tile>,
    pub discarded: Vec<Tile>,
    pub riichi: bool,
}

impl Player {
    pub fn evaluate_yakus(self, prevalent_wind: Wind) -> Vec<Yaku> {
        let mut yakus = vec![];

        let wind_tiles: Vec<Tile> = self
            .hand
            .iter()
            .cloned()
            .filter(|x| match x {
                Tile::Wind(_) => true,
                _ => false,
            })
            .collect();

        if wind_tiles.len() >= 3 {
            for wind_type in [Wind::East, Wind::North, Wind::South, Wind::West] {
                let count = wind_tiles
                    .iter()
                    
                    .filter(|x| **x == Tile::Wind(wind_type))
                    .count();
                
                if count >= 3 && wind_type == prevalent_wind {
                    yakus.push(Yaku::PrevalentWind(wind_type));
                } else if count >= 3 && wind_type == self.seat {
                    yakus.push(Yaku::PrevalentWind(wind_type));
                }
            }
        }

        yakus
    }

    pub fn is_menzenchin(self) -> bool {
        return self.dealt_in.is_empty();
    }
}