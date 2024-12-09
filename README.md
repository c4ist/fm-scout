# FM24 Scout - Hidden Gems Finder 
A high-performance scouting tool for Football Manager 2024 that helps you find hidden gems and analyze potential signings. Built with Rust for maximum speed and efficiency.

## Features

- Lightning-fast analysis of player data using parallel processing
- Smart scoring system based on position-specific attributes
- Beautiful colored console output for easy reading
- Memory-efficient processing of large datasets
- Customizable search criteria

## Prerequisites

1. Install Rust:
   - Windows: Download and run rustup-init.exe from https://rustup.rs/
   - Follow the installation prompts

## Setup

1. Clone or download this repository
2. Open a terminal in the project directory
3. Build the project:
```bash
cargo build --release
```

## Usage

Export your FM24 data as CSV, then run:

```bash
cargo run --release -- -f path/to/your/data.csv -p ST -a 23 -v 5.0 -m 130
```

Arguments:
- `-f, --file`: Path to your FM24 exported CSV file
- `-p, --position`: Position to search for (ST, CM, CB, etc.)
- `-a, --max-age`: Maximum age to consider (default: 23)
- `-v, --max-value`: Maximum value in millions (default: 5.0)
- `-m, --min-potential`: Minimum potential ability (default: 130)

## Data Requirements

Your CSV export should include these columns:
- name
- age
- club
- nationality
- position
- value
- wage
- current_ability
- potential_ability
- Technical attributes: finishing, first_touch, passing, technique, dribbling, tackling
- Mental attributes: decisions, anticipation, composure, vision, work_rate
- Physical attributes: acceleration, pace, stamina, strength, jumping

## Tips for Finding Gems

1. Use lower max_value for finding bargains
2. Focus on high min_potential (140+) for future stars
3. Adjust max_age based on your development strategy
4. Consider position-specific searches for better results
