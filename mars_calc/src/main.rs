use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input);

    borrow_string(&input);
    own_string(input)
    
    // println!("Input: {}", input);
    // let mars_weight: f32 = calculate_mars_weight(30.0);
    // println!("Weight on mars: {}", mars_weight);
}

fn calculate_mars_weight(earth_weight: f32) -> f32 {
    earth_weight/ 9.81 * 3.711
}

fn borrow_string(s: &String)  {
    println!("{}", s);
}

fn own_string(s: String)  {
    println!("{}", s);
}
