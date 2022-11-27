use std::io;

fn main() {
    // take user input (weight in lbs.)
    println!("Enter your weight (lbs): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();    
    
    // take input (string) and convert to float then unwrap its value
    let weight: f64 = input.trim().parse().unwrap();

    // calculate weight on mars and print result
    println!("You entered: {}", input);
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Your weight on Mars is: {}", mars_weight);
}

// accepts weight in pounds (on earth) and returns weight on the moon
fn calculate_weight_on_mars(weight: f64) -> f64 {
    (weight / 9.81) * 3.711
}

// passing references -> pass pointers (borrowing)
// pass values -> move ownership

