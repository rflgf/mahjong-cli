use std::io;

use crate::{player::Player, tile::{Tile, Wind}};

pub mod player;
pub mod tile;
pub mod yaku;

fn main() {
    loop {
        println!("options:");
        println!("\t1 for a randomly populated hand");
        println!("\t2 to manually populate a hand");
        println!("\tany other input to quit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mode: u8 = input.trim_end().parse().unwrap();

        match mode {
            1 => {
                println!("randomly populated not yet available!");

                let player = Player {
                    seat: Wind::East,
                    hand: vec![
                        Tile::Wind(Wind::East),
                        Tile::Wind(Wind::East),
                        Tile::Wind(Wind::East),
                    ],
                    dealt_in: vec![],
                    discarded: vec![],
                    riichi: false,
                };

                print!("hand:\n\t");
                for tile in &player.hand {
                    print!("{}, ", tile);
                }
                println!();

                print!("yakus:\n\t");
                for yaku in player.evaluate_yakus(Wind::East) {
                    print!("{}, ", yaku);
                }
                println!();
            }
            2 => {
                println!("manually populated not yet available!");
            }
            _ => {
                break;
            }
        }
    }
}
