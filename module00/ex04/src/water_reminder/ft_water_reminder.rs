use std::io;

pub fn ft_water_reminder(){
    let mut input = String::new();

    println!("Days since last watering: ");
    io::stdin().read_line(&mut input).expect("fail");
    let days : i32 = input.trim().parse().expect("please put a number");
    input.clear();

    if days > 2 {
        println!("Water the plants!");
    }
    else {
        println!("Plants are fine!");
    }
}
