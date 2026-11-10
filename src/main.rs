#![allow(dead_code, unused_variables, unused_mut)]

//! Entry point for Chapter 11.2: Controlling How Tests Are Run.
//!
//! This binary keeps the same small dungeon rules as the previous demo. The
//! tests demonstrate filtering, captured output, ignored tests, and safe use
//! of shared resources while the test harness runs tests in parallel.

fn main() {
    let command = parse_command("go north").expect("the example command is valid");
    println!("Parsed command: {command:?}");

    let mut player = Character {
        name: String::from("Aria"),
        hp: 8,
        max_hp: 20,
    };
    heal(&mut player, 5);

    println!("After healing: {player:?}");
    println!("Damage after armor: {}", damage_after_armor(6, 2));
}

/// A command understood by the dungeon rules.
#[derive(Debug, PartialEq, Eq)]
enum Command {
    Go(String),
    Heal(u32),
}

/// Parse one command, returning a useful error instead of panicking.
fn parse_command(input: &str) -> Result<Command, &'static str> {
    let mut words = input.split_whitespace();
    let verb = words.next().ok_or("command cannot be empty")?;

    match verb {
        "go" => {
            let direction = words.next().ok_or("go needs a direction")?;
            if words.next().is_some() {
                return Err("go takes one direction");
            }
            Ok(Command::Go(String::from(direction)))
        }
        "heal" => {
            let points = words
                .next()
                .ok_or("heal needs an amount")?
                .parse::<u32>()
                .map_err(|_| "healing amount must be a whole number")?;
            if words.next().is_some() {
                return Err("heal takes one amount");
            }
            Ok(Command::Heal(points))
        }
        _ => Err("unknown command"),
    }
}

/// A player whose complete state can be compared in an assertion.
#[derive(Debug, PartialEq, Eq)]
struct Character {
    name: String,
    hp: u32,
    max_hp: u32,
}

/// Armor reduces a hit, but every hit deals at least one point of damage.
fn damage_after_armor(incoming: u32, armor: u32) -> u32 {
    incoming.saturating_sub(armor).max(1)
}

/// Restore health without allowing it to exceed the character's maximum.
fn heal(character: &mut Character, points: u32) {
    character.hp = character.hp.saturating_add(points).min(character.max_hp);
}

/// Convert a percentage of maximum health to healing points.
fn healing_from_percent(max_hp: u32, percent: u32) -> u32 {
    assert!(percent <= 100, "percent must be between 0 and 100");
    ((u64::from(max_hp) * u64::from(percent)) / 100) as u32
}

/// Whether a roll is a critical hit.
fn is_critical(roll: u32) -> bool {
    roll >= 95
}

// `cfg(test)` keeps test code out of the normal binary build.
#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn damage_never_drops_below_one() {
        assert_eq!(damage_after_armor(3, 100), 1);
    }

    #[test]
    fn armor_changes_the_damage_value() {
        assert_ne!(damage_after_armor(8, 2), 8);
    }

    #[test]
    fn critical_hits_start_at_ninety_five() {
        assert!(is_critical(95));
        assert!(!is_critical(94));
    }

    #[test]
    fn healing_stops_at_maximum_health() {
        let mut player = Character {
            name: String::from("Aria"),
            hp: 18,
            max_hp: 20,
        };

        heal(&mut player, 5);

        assert_eq!(player.hp, 20);
    }

    #[test]
    fn command_parsing_can_use_question_mark() -> Result<(), &'static str> {
        let command = parse_command("go north")?;

        assert_eq!(command, Command::Go(String::from("north")));
        Ok(())
    }

    #[test]
    #[should_panic(expected = "percent must be between 0 and 100")]
    fn healing_percentage_over_one_hundred_panics() {
        healing_from_percent(100, 101);
    }

    // Passing-test output is captured by default. Try:
    // cargo test -- --show-output
    // cargo test -- --nocapture
    #[test]
    fn passing_test_output_is_captured_by_default() {
        println!("This output is hidden unless the harness is asked to show it.");
        assert_eq!(damage_after_armor(6, 2), 4);
        // Uncomment to see the failure report and captured output:
        // assert_eq!(10, 5);
    }

    #[test]
    fn save_file_round_trips_aria() -> Result<(), Box<dyn Error>> {
        let path = save_path("aria");
        fs::write(&path, "Aria")?;

        let saved_name = fs::read_to_string(&path)?;
        assert_eq!(saved_name, "Aria");

        fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    fn save_file_round_trips_borin() -> Result<(), Box<dyn Error>> {
        let path = save_path("borin");
        fs::write(&path, "Borin")?;

        let saved_name = fs::read_to_string(&path)?;
        assert_eq!(saved_name, "Borin");

        fs::remove_file(path)?;
        Ok(())
    }

    // Cargo passes test-harness arguments after `--`, for example:
    // cargo test damage
    // cargo test -- --list
    // cargo test -- --test-threads=1
    // cargo test -- --ignored
    // cargo test -- --include-ignored
    //
    // This package currently has one binary target:
    // cargo test --bin rust-1002-controlling-how-tests-run
    // A library, doc tests, and integration tests arrive in the next demo, so
    // their target selectors have nothing to select yet.

    #[test]
    #[ignore = "extended fifty-million-turn rule check"]
    fn extended_fight_keeps_health_in_range() {
        let mut hp: u32 = 100;
        let max_hp = 100;

        for turn in 0..50_000_000 {
            let incoming = turn % 8;
            let armor = turn % 4;
            hp = hp
                .saturating_sub(damage_after_armor(incoming, armor))
                .max(1);

            if turn % 10 == 0 {
                hp = hp.saturating_add(5).min(max_hp);
            }

            assert!((1..=max_hp).contains(&hp));
        }
    }

    /// Give each test its own file so parallel tests cannot overwrite one
    /// another's data. Replacing these names with one shared path can make
    /// tests interfere; separate state is preferable to serializing the suite.
    fn save_path(test_name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("rust-1002-{}-{test_name}.txt", std::process::id()))
    }
}
