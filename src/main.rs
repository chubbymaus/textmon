mod utils;

use colored::*;
use utils::select_pokemon;

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
        "{} vs {} Let the battle begin!",
        selected_pokemon.blue(),
        selected_pokemon_to_fight.blue()
    );
}
