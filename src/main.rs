use colored::*;
use std::io;

fn select_pokemon() -> String {
    let options = [
        "Charizard",
        "Blastoise",
        "Lucario",
        "Mewtwo",
        "Rayquaza",
        "Zapdos",
    ];
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: usize = choice.trim().parse().expect("Please enter a number");

    let selected_pokemon = match choice {
        1 => options[0],
        2 => options[1],
        3 => options[2],
        4 => options[3],
        5 => options[4],
        6 => options[5],
        _ => {
            println!("Invalid choice, defaulting to Charizard");
            options[0]
        }
    };

    println!("You selected: {}\n", selected_pokemon);
    selected_pokemon.to_string()
}
fn main() {
    let title_text = r#"
 _____         _                         
|_   _|____  _| |_ _ __ ___   ___  _ __  
  | |/ _ \ \/ / __| '_ ` _ \ / _ \| '_ \ 
  | |  __/>  <| |_| | | | | | (_) | | | |
  |_|\___/_/\_\\__|_| |_| |_|\___/|_| |_|
                                        
  "#;
    println!("{}", title_text.green());
    let welcome_text = "Please select your team: ";
    println!("{}", welcome_text.bright_cyan());

    let selected_pokemon = select_pokemon();

    let fight_text = "Please select the pokemon you want to fight: ";
    println!("{}", fight_text.bright_cyan());
    let selected_pokemon_to_fight = select_pokemon();

    println!("You selected: {}\n", selected_pokemon_to_fight);
    println!(
        "{} vs {}! Let the battle begin!",
        selected_pokemon.blue(),
        selected_pokemon_to_fight.blue()
    );
}
