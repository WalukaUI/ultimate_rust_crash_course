use std::io;   
    
const  STARTING_MISSILES: i32 =8;
const  READY_AMOUNT: i32 =2;

fn main() {
let mut missiles=8;
let ready=2;
println!("Firing {} of my {} missiles...", ready, missiles);
missiles = missiles - ready;
println!("{} missiles left", missiles);
println!("{} missiles left {}",STARTING_MISSILES, READY_AMOUNT);

    println!("Guess the number!");

println!("Please input your guess.");

let mut guess = String::new();

io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

println!("You guessed: {guess}");
}
