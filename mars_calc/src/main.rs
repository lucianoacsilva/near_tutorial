use std::io;

fn main() {
    println!("User, enter your weight in kilograms!");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    
    let mars_weight: f32 = calculate_mars_weight(weight);
    println!("Weight on mars: {}", mars_weight);
}

fn calculate_mars_weight(earth_weight: f32) -> f32 {
    earth_weight/ 9.81 * 3.711
}