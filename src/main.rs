mod cards;

use cards::Cards;
use clap::Parser;

/// Program to simulate Gloomhaven Decks
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Starting hand
   #[clap(short, long, value_parser, default_value_t = 10)]
   starting_hand: u8,
}

fn main() {
    let args = Args::parse();

    let mut hand = Cards::new(args.starting_hand);
    println!("This deck should last {} turns", hand.calculate_maximum_rounds());

    let mut turn: u8 = 1;
    println!("Starting with {}", hand);
    while let Ok(()) = hand.play_round(0) {
        println!("turn {}: {}", turn, hand);
        turn += 1;
    }
    println!("Character lasted {} turns ", turn - 1);
}