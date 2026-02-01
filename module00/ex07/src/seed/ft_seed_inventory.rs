pub fn ft_seed_inventory(seed_type: &str, quantity: i32, unit: &str) {
    let mut chars = seed_type.chars();
    let first_word = match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    };

    match unit {
        "packets" => println!("{} seeds: {} packets available", first_word, quantity),
        "grams" => println!("{} seeds: {} grams total", first_word, quantity),
        "area" => println!("{} seeds: covers {} square meters", first_word, quantity),
        _ => println!("Unknown unit type"),
    }
}