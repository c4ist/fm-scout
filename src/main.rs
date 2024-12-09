use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use rayon::prelude::*;
use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Player {
    name: String,
    age: u8,
    club: String,
    nationality: String,
    position: String,
    value: f64,
    wage: f64,
    current_ability: u8,
    potential_ability: u8,
    // Technical attributes
    finishing: u8,
    first_touch: u8,
    passing: u8,
    technique: u8,
    dribbling: u8,
    tackling: u8,
    // Mental attributes
    decisions: u8,
    anticipation: u8,
    composure: u8,
    vision: u8,
    work_rate: u8,
    // Physical attributes
    acceleration: u8,
    pace: u8,
    stamina: u8,
    strength: u8,
    jumping: u8,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    
    #[arg(short, long)]
    file: PathBuf,

    #[arg(short, long, default_value = "23")]
    max_age: u8,

    #[arg(short, long, default_value = "5.0")]
    max_value: f64,
    
    #[arg(short, long, default_value = "130")]
    min_potential: u8,

    #[arg(short, long)]
    position: String,
}

impl Player {
    fn calculate_score(&self, position: &str) -> f64 {
        let position_weights = match position.to_uppercase().as_str() {
            "ST" => vec![
                (self.finishing, 2.0),
                (self.first_touch, 1.5),
                (self.acceleration, 1.5),
                (self.pace, 1.5),
                (self.composure, 1.0),
            ],
            "CM" => vec![
                (self.passing, 2.0),
                (self.vision, 1.5),
                (self.decisions, 1.5),
                (self.stamina, 1.0),
                (self.work_rate, 1.5),
            ],
            "CB" => vec![
                (self.tackling, 2.0),
                (self.strength, 1.5),
                (self.jumping, 1.5),
                (self.anticipation, 1.5),
                (self.decisions, 1.0),
            ],
            _ => vec![
                (self.technique, 1.0),
                (self.decisions, 1.0),
                (self.stamina, 1.0),
                (self.strength, 1.0),
                (self.work_rate, 1.0),
            ],
        };

        let attribute_score: f64 = position_weights.iter()
            .map(|(attr, weight)| *attr as f64 * weight)
            .sum::<f64>() / position_weights.iter().map(|(_, w)| w).sum::<f64>();

        let potential_score = self.potential_ability as f64 / 200.0;
        let value_score = 1.0 - (self.value.min(50_000_000.0) / 50_000_000.0);
        
        (attribute_score * 0.4) + (potential_score * 0.4) + (value_score * 0.2)
    }
}

fn find_gems(
    players: &[Player],
    args: &Args,
) -> Vec<Player> {
    let pb = ProgressBar::new(players.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap());

    let mut filtered: Vec<_> = players.par_iter()
        .filter(|p| {
            pb.inc(1);
            p.age <= args.max_age &&
            p.value <= args.max_value * 1_000_000.0 &&
            p.potential_ability >= args.min_potential &&
            p.position.to_lowercase().contains(&args.position.to_lowercase())
        })
        .cloned()
        .collect();

    pb.finish_with_message("Analysis complete");

    filtered.sort_by(|a, b| {
        let score_a = a.calculate_score(&args.position);
        let score_b = b.calculate_score(&args.position);
        score_b.partial_cmp(&score_a).unwrap()
    });

    filtered
}

fn display_player(player: &Player, position: &str) {
    println!("\n{}", "=".repeat(50).yellow());
    println!("{}", player.name.bright_green().bold());
    println!("{}", "=".repeat(50).yellow());
    
    println!("Club: {}", player.club.cyan());
    println!("Age: {}", player.age.to_string().cyan());
    println!("Value: €{:.2}M", player.value / 1_000_000.0);
    println!("Wage: €{:.2}K/week", player.wage / 1_000.0);
    println!("Current Ability: {}", player.current_ability.to_string().yellow());
    println!("Potential Ability: {}", player.potential_ability.to_string().bright_yellow());
    
    println!("\n{}", "Key Attributes:".underline());
    match position.to_uppercase().as_str() {
        "ST" => {
            println!("Finishing: {}", player.finishing);
            println!("First Touch: {}", player.first_touch);
            println!("Acceleration: {}", player.acceleration);
            println!("Pace: {}", player.pace);
            println!("Composure: {}", player.composure);
        },
        "CM" => {
            println!("Passing: {}", player.passing);
            println!("Vision: {}", player.vision);
            println!("Decisions: {}", player.decisions);
            println!("Stamina: {}", player.stamina);
            println!("Work Rate: {}", player.work_rate);
        },
        "CB" => {
            println!("Tackling: {}", player.tackling);
            println!("Strength: {}", player.strength);
            println!("Jumping: {}", player.jumping);
            println!("Anticipation: {}", player.anticipation);
            println!("Decisions: {}", player.decisions);
        },
        _ => {
            println!("Technique: {}", player.technique);
            println!("Decisions: {}", player.decisions);
            println!("Work Rate: {}", player.work_rate);
            println!("Stamina: {}", player.stamina);
        }
    }
    
    println!("\nOverall Score: {:.2}", player.calculate_score(position));
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("{}", "\nFM24 Scout - Hidden Gems Finder".bright_blue().bold());
    println!("{}", "=".repeat(50).blue());

    let file = File::open(&args.file)?;
    let mut rdr = csv::Reader::from_reader(file);
    
    println!("Loading and analyzing player data...");
    
    let players: Vec<Player> = rdr.deserialize()
        .filter_map(Result::ok)
        .collect();

    let gems = find_gems(&players, &args);

    println!("\nFound {} potential signings:", gems.len());

    for (i, player) in gems.iter().take(10).enumerate() {
        println!("\n{}. {}", i + 1, "Recommendation".bright_purple());
        display_player(player, &args.position);
    }

    Ok(())
}
