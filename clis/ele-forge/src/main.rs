use std::env;
use std::str::FromStr;

use ele_forge::*;

fn print_elements(lines: &mut Vec<String>, ele: &Ele, forge: &Forge, indent: usize) {
    let cost = ele.cost(forge);
    let price_or_forge = match ele.price() {
        Some(p) => format!("Price: {}", p.to_string()),
        None => {
            let recipe = forge.find_recipe(&format!("{}", ele)).unwrap();
            for &ele2 in recipe {
                print_elements(lines, &ele2, forge, indent + 1);
            }
            format!("Forge: {:?} Price: {:?}", recipe, recipe.price())
        }
    };

    let indent_str = " ".repeat(indent * 4);
    lines.push(format!(
        "{}--> Element: {}, Cost: {}, {}",
        indent_str, ele, cost, price_or_forge
    ));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <element> <quantity>", args[0]);
        return;
    }

    let element_name = &args[1];
    let quantity: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("The second argument must be a number.");
            return;
        }
    };

    let forge = Forge::default();

    if let Some(recipe) = forge.find_recipe(element_name) {
        let total_cost: u32 = recipe.cost(&forge) * quantity;
        let total_price = match Ele::from_str(element_name) {
            Ok(ele) => ele.price().and_then(|p| Some(p * quantity)),
            Err(_) => None,
        };

        println!(
            "The cost of {} ({}) is {}, or buy it for {:?}",
            element_name, quantity, total_cost, total_price
        );

        println!("Required elements (1):");
        for &ele in recipe {
            let mut lines = Vec::new();
            print_elements(&mut lines, &ele, &forge, 0);
            lines.reverse();

            for line in lines {
                println!("{}", line);
            }
        }
    } else {
        println!("No recipe found for {}", element_name);
    }
}
