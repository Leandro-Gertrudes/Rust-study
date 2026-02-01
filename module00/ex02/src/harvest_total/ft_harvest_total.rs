use std::io;

pub fn ft_harvest_total()
{
    let mut input = String::new();
    let mut count : i32 = 0; 

    for n in 1..4{
        println!("Day {} harvest:", n);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let value: i32 = input.trim().parse().expect("please enter a number");
        count += value;
        input.clear();
    }
    println!("Total harvest: {}", count);
}