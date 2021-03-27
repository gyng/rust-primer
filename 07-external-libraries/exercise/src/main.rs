// Use clap and serde_json to parse and pretty-print user information
// These are the defacto libraries, but are heavyweight and will add to compile time

// use clap::Clap;
// use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

// CLI args object: derive clap on this
// https://github.com/clap-rs/clap#using-derive-macros
struct MyOpts {
    input: String,
}

// https://docs.serde.rs/serde_json/#parsing-json-as-strongly-typed-data-structures
// Add Serialize, Deserialize to derive JSON conversion
#[derive(Debug)]
struct ProfilePreferences {
    food: Vec<String>,
}

https://docs.serde.rs/serde_json/#parsing-json-as-strongly-typed-data-structures
// Add Serialize, Deserialize to derive JSON conversion
#[derive(Debug)]
struct Profile {
    age: i32,
    weight: Option<f32>,
    preferences: ProfilePreferences,
}

// Run this with `cargo run -- -i input.json`
fn main() {
    // Read CLI args
    // https://github.com/clap-rs/clap#quick-example
    // Derive is the easiest
    let opts: MyOpts = MyOpts::parse();
    dbg!(&opts);

    // Read the file https://doc.rust-lang.org/book/ch12-02-reading-a-file.html

    // Parse the JSON
    let profile: Profile = serde_json::from_str(&contents)?;
    dbg!(&profile);

    // Print it!
    println!("{}", emoji_food_preferences(&profile));
}

fn emoji_food_preferences(profile: &Profile) -> String {
    let mut mapping: HashMap<String, &str> = HashMap::new();
    mapping.insert("taco".into(), "🌮");
    mapping.insert("pizza".into(), "🍕");
    mapping.insert("cookie".into(), "🍪");

    let mut emojified_food: Vec<String> = vec![];

    for food in profile.preferences.food.iter() {
        if let Some(ref emoji) = mapping.get(food) {
            emojified_food.push(emoji.to_string())
        } else {
            emojified_food.push(food.to_string());
        }
    }

    emojified_food.join(" ")
}
