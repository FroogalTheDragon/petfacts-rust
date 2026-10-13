use clap::Parser;
use rand::Rng;
use reqwest::{self, Result};
use std::{
    io::{Write, stdout},
    thread,
    time::Duration,
};

const DOG_API_URL: &str = "https://dogapi.dog/api/v2/facts";
const CAT_API_URL: &str = "https://catfact.ninja/fact";

/// Prints out a cat or dog fact, it no animal is specified a random one will be chosen.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Print a cat fact
    #[arg(long, short, action)]
    cat: bool,

    /// Print a dog fact
    #[arg(long, short, action)]
    dog: bool,
}

/// Gets a dog fact from dog fact API
fn get_dog_fact() -> Result<String> {
    let json_data = reqwest::blocking::get(DOG_API_URL)?.text()?;
    let dog_fact = json::parse(&json_data).expect("Failed to parse dog fact JSON string");
    Ok(dog_fact["data"][0]["attributes"]["body"].to_string())
}

/// Gets a cat fact from the cat fact API
fn get_cat_fact() -> Result<String> {
    let json_data = reqwest::blocking::get(CAT_API_URL)?.text()?;
    let cat_fact = json::parse(&json_data).expect("Failed to parse cat fact JSON string");
    Ok(cat_fact["fact"].to_string())
}

/// Return dog or cat as a string randomly
fn random_animal() -> &'static str {
    if rand::rng().random::<bool>() {
        "dog"
    } else {
        "cat"
    }
}

/// Give the output an effect of being typed out
fn type_fact(string_to_type: &String) {
    for character in string_to_type.chars() {
        print!("{character}");
        stdout().flush().expect("Something is wrong with stdout!");
        thread::sleep(Duration::from_millis(50));
    }
    println!(""); // Go to the next line when done "typing"
}

/// Print a pet fact based on the animal passed
fn print_fact(animal: Option<&str>) -> Result<()> {
    match animal {
        Some("dog") => {
            let dog_fact = get_dog_fact()?;
            type_fact(&dog_fact);
            Ok(())
        }
        Some("cat") => {
            let cat_fact = get_cat_fact()?;
            type_fact(&cat_fact);
            Ok(())
        }
        Some(&_) => {
            panic!("Error in the print_fact function");
        }
        None => {
            let animal = random_animal();
            if animal == "cat" {
                let cat_fact = get_cat_fact()?;
                type_fact(&cat_fact);
            } else {
                let dog_fact = get_dog_fact()?;
                type_fact(&dog_fact);
            }
            Ok(())
        }
    }
}

fn main() {
    let args = Args::parse();

    if args.cat {
        println!("Cat");
        print_fact(Some("cat")).unwrap_or_else(|e| {
            panic!("Failed to print cat fact: {}", e);
        });
    } else if args.dog {
        println!("Dog");
        print_fact(Some("dog")).unwrap_or_else(|e| {
            panic!("Failed to print dog fact: {}", e);
        });
    } else {
        println!("Random");
        print_fact(None).unwrap_or_else(|e| {
            panic!("Failed to print randomized fact: {}", e);
        });
    }
}
