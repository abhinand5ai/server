use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your weight: ");
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let mut mars_weight = calculate_weight_on_mars(weight);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars {} kg", mars_weight)
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn borrow_string(s: &String) {
    println!("{}", s)
}

fn own_string(s: String){
    println!("{}", s);
}