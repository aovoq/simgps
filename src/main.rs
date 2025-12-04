use std::io::{stdout, Write};
use std::process::Command;

use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};

const DEFAULT_STEP_METERS: f64 = 10.0;
const METERS_PER_DEGREE: f64 = 111_000.0; // approximate

#[derive(Parser)]
#[command(name = "simgps")]
#[command(about = "iOS Simulator GPS Controller")]
#[command(arg_required_else_help = true)]
struct Args {
    /// Coordinates (e.g., "35.66, 139.69" or "35.66 139.69")
    #[arg(num_args = 1..)]
    coords: Vec<String>,

    /// Step size in meters per key press
    #[arg(short, long, default_value_t = DEFAULT_STEP_METERS)]
    step: f64,
}

fn parse_coords(input: &[String]) -> Result<(f64, f64), String> {
    // Join all arguments and normalize separators
    let joined = input.join(" ");

    // Remove commas and extra spaces, then split
    let parts: Vec<String> = joined
        .replace(',', " ")
        .split_whitespace()
        .map(String::from)
        .collect();

    if parts.len() != 2 {
        return Err(format!(
            "Expected 2 coordinates, got {}. Usage: simgps <lat> <lng>",
            parts.len()
        ));
    }

    let lat = parts[0]
        .parse::<f64>()
        .map_err(|_| format!("Invalid latitude: {}", parts[0]))?;
    let lng = parts[1]
        .parse::<f64>()
        .map_err(|_| format!("Invalid longitude: {}", parts[1]))?;

    Ok((lat, lng))
}

fn set_location(lat: f64, lng: f64) {
    let _ = Command::new("xcrun")
        .args(["simctl", "location", "booted", "set", &format!("{},{}", lat, lng)])
        .output();
}

fn draw(lat: f64, lng: f64, step_meters: f64, last_action: &str) {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0), Clear(ClearType::All)).unwrap();
    print!("simgps - iOS Simulator GPS Controller\r\n");
    print!("=====================================\r\n");
    print!("\r\n");
    print!("Latitude:  {:.15}\r\n", lat);
    print!("Longitude: {:.15}\r\n", lng);
    print!("Step:      {}m\r\n", step_meters);
    print!("\r\n");
    print!("Last: {}\r\n", last_action);
    print!("\r\n");
    print!("[Arrow Keys] Move  [q/Esc] Quit\r\n");
    stdout.flush().unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let (mut lat, mut lng) = parse_coords(&args.coords).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    let step_meters = args.step;
    let step = step_meters / METERS_PER_DEGREE;
    let mut last_action = String::from("Ready");

    // Initial location set
    set_location(lat, lng);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    // Initial draw
    draw(lat, lng, step_meters, &last_action);

    // Main loop
    loop {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = read()? {
            // Handle Ctrl+C
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                break;
            }

            match code {
                KeyCode::Up => {
                    lat += step;
                    set_location(lat, lng);
                    last_action = String::from("^ North");
                    draw(lat, lng, step_meters, &last_action);
                }
                KeyCode::Down => {
                    lat -= step;
                    set_location(lat, lng);
                    last_action = String::from("v South");
                    draw(lat, lng, step_meters, &last_action);
                }
                KeyCode::Right => {
                    lng += step;
                    set_location(lat, lng);
                    last_action = String::from("> East");
                    draw(lat, lng, step_meters, &last_action);
                }
                KeyCode::Left => {
                    lng -= step;
                    set_location(lat, lng);
                    last_action = String::from("< West");
                    draw(lat, lng, step_meters, &last_action);
                }
                KeyCode::Char('q') | KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    // Cleanup
    execute!(stdout, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
