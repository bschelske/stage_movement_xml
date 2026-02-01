use std::io;

fn main() {
    welcome_message();

    let mut running: bool = true;
    while running {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input = user_input.trim();

        match user_input {
            "q" => {
                running = false;
                println!("Quitting...");
            }
            "" => {
                println!("enter")
            }
            _ => {
                println!("Unknown input: {}", user_input);
            }
        }
    }
}

fn welcome_message() {
    println!(
        r"
     _                                                                     _   
 ___| |_ __ _  __ _  ___     _ __ ___   _____   _____ _ __ ___   ___ _ __ | |_ 
/ __| __/ _` |/ _` |/ _ \   | '_ ` _ \ / _ \ \ / / _ \ '_ ` _ \ / _ \ '_ \| __|
\__ \ || (_| | (_| |  __/   | | | | | | (_) \ V /  __/ | | | | |  __/ | | | |_ 
|___/\__\__,_|\__, |\___/   |_| |_| |_|\___/ \_/ \___|_| |_| |_|\___|_| |_|\__|
              |___/                                                        
XML Generator
Ben 2026
-------------------------------------------------------------------------------------------
"
    );
    println!("Instructions:");
    println!("Follow the prompts and press enter to continue.");
    println!("Enter 'q' to quit.");
}

fn calculate_stepsize(distance: f64, steps: u32) -> f64 {
    distance / steps as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_stepsize() {
        let distance = 10.0;
        let steps = 5;
        let expected_stepsize = 2.0;
        let result = calculate_stepsize(distance, steps);
        assert_eq!(result, expected_stepsize);
    }
}

