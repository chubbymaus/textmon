use std::io;

pub fn select_pokemon() -> String {
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

pub struct MoveList {
    name: str
}
pub struct Pokemon {
    name: str,
    health_points: u32,
    move_list: MoveList,
}