use std::io;

pub fn ft_plant_age(){
    let mut input = String::new();

    println!("Enter plant age in days: ");
    io::stdin().read_line(&mut input).expect("fail");
    let days : i32 = input.trim().parse().expect("please put a number");

    if days > 60 {
        println!("Plant is ready to harvest!");
    }
    else {
        println!("Plant needs more time to grow.");
    }
}