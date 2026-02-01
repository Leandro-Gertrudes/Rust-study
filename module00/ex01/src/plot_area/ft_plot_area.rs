use std::io;

pub fn ft_plot_area()
{
    let mut input = String::new();

    println!("Enter length: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let length: i32 = input.trim().parse().expect("please enter a number");
    input.clear();

    println!("Enter width: ");
    io::stdin().read_line(&mut input).expect("failed to read line");
    let width: i32 = input.trim().parse().expect("please enter a number");
    input.clear();

    let area = width * length;
    println!("plot area: {}", area);
}