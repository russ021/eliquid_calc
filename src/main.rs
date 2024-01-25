use std::io;

fn main() {
    // Get user input
    let base_amount = get_user_input("Enter the amount of base liquid (in ml):");
    let target_nicotine = get_user_input("Enter the desired nicotine strength (in mg/ml):");
    let target_volume = get_user_input("Enter the target final volume (in ml):");

    // Calculate amounts
    let nicotine_base_amount = calculate_nicotine_base_amount(
        base_amount, target_nicotine, target_volume);
    let flavoring_amount = calculate_flavoring_amount(target_volume);
    let diluent_amount = calculate_diluent_amount(
        base_amount, nicotine_base_amount, flavoring_amount, target_volume);

    // Display results
    println!("Nicotine Base Amount: {} ml", nicotine_base_amount);
    println!("Flavoring Amount: {} ml", flavoring_amount);
    println!("Diluent Amount: {} ml", diluent_amount);
}

fn get_user_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn calculate_nicotine_base_amount(base_amount: f64, target_nicotine: f64, target_volume: f64) -> f64 {
    (target_nicotine / 1000.0) * target_volume - base_amount
}

fn calculate_flavoring_amount(target_volume: f64) -> f64 {
    // You can customize this function based on your specific flavoring concentration
    // For simplicity, let's assume 5% flavoring for this example
    0.05 * target_volume
}

fn calculate_diluent_amount(
    base_amount: f64, nicotine_base_amount: f64, flavoring_amount: f64, target_volume: f64) -> f64 {
    target_volume - (base_amount + nicotine_base_amount + flavoring_amount)
}

