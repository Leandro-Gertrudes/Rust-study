use std::io;

pub fn ft_garden_summary(){
    let mut name = String::new();
    let mut plants = String::new();

    println!("Enter garden name: ");
    io::stdin().read_line(&mut name).expect("fail");
    println!("Enter number of plants: ");
    io::stdin().read_line(&mut plants).expect("fail");

    println!("Garden: {}", name.trim());
    println!("Plants: {}", plants.trim());
    println!("Status: Growing well!");
}