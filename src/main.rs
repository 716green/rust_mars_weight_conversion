use std::io;

fn main() {
    let mut input = String::new();

    println!("How much do you weigh on earth? (kg)");
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    dbg!(weight);


    println!("Input: {}", input);

    let mars_weight = calculate_weight_on_mars(weight);
    let mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {}g", mars_weight);
}


fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}