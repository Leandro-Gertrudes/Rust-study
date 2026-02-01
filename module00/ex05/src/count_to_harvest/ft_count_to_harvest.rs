use std::io;

pub fn ft_count_to_harvest_iterative(){
    let mut input = String::new();
    
    println!("Days until harvest: ");
    io::stdin().read_line(&mut input).expect("fail");
    let count: i32 = input.trim().parse().expect("please put a number");
    input.clear();

    for n in 1..count + 1{
        println!("day {}", n);
    }
    println!("Harvest time!");
    println!("");
}

fn recursive(count: i32,mut n: i32){
    println!("day: {}", n);
    n += 1;
    if n <= count {
        recursive(count, n);
    }
}

pub fn ft_count_to_harvest_recursive(){
    let mut input = String::new();
    
    println!("Days until harvest: ");
    io::stdin().read_line(&mut input).expect("fail");
    let count: i32 = input.trim().parse().expect("please put a number");
    input.clear();

    recursive(count, 1);
    println!("Harvest time!");
}