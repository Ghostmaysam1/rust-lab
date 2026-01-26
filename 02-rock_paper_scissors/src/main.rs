use rand;
use std::io;

fn main() {
    loop {
        let mut user_input = String::new();
        let computer = rand::random_range(1..=3);
        println!("Please Enter a number");
        println!("1 for 🗿 Rock!");
        println!("2 for 📄 Paper!");
        println!("3 for ✂️  Scissors!");

        // read the input
        io::stdin().read_line(&mut user_input).unwrap();
        user_input = user_input.trim().to_lowercase();

        if user_input == String::from("exit") {
            break;
        }

        let user_input: i8 = user_input.parse().unwrap_or_else(|error| {
            println!("{error}");
            0
        });

        println!("{}", check_values(&computer, &user_input));

        println!("------------------------------------------------");
    }
}

fn check_values(computer: &i8, user: &i8) -> String {
    let emoji = match computer {
        1 => "🗿 Rock",
        2 => "📄 Paper",
        3 => "✂️  Scissors",
        _ => "Invalid Move by Computer",
    };
    if user == computer {
        format!("Draw — both players chose the {emoji}")
    } else if check_game(computer, user) {
        format!("Victory — your move beats the computer. computer chose {emoji}")
    } else {
        format!("Defeat — the computer outplayed you. computer chose {emoji}")
    }
}

fn check_game(computer: &i8, user: &i8) -> bool {
    if (*computer == 3 && *user == 1)
        || (*computer == 2 && *user == 3)
        || (*computer == 1 && *user == 2)
    {
        true
    } else {
        false
    }
}
