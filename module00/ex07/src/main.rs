mod seed;
use seed::ft_seed_inventory::ft_seed_inventory;

fn main() {
    ft_seed_inventory("tomato", 15, "packets");
    ft_seed_inventory("carrot", 8, "grams");
    ft_seed_inventory("lettuce", 12, "area");
    ft_seed_inventory("grapes ", 7, "grams");
    ft_seed_inventory("grapes ", 7, "kg");
}
