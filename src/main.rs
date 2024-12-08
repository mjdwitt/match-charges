use std::io;

use color_eyre::eyre::Result;

mod matcher;
mod types;

use types::{Charge, Order};

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Input orders [order name: 10.99]:");
    let mut orders: Vec<Order> = Vec::new();
    for line in io::stdin().lines() {
        let line = line?;
        if line == "" {
            println!();
            break;
        }
        orders.push(line.parse()?)
    }

    println!("Input charges [charge date: 2.45]:");
    let mut charges: Vec<Charge> = Vec::new();
    for line in io::stdin().lines() {
        let line = line?;
        if line == "" {
            println!();
            break;
        }
        charges.push(line.parse()?)
    }

    let mut solutions = 0;
    for (n, solution) in matcher::match_charges::<Order, Charge>(&orders, &mut charges)
        .into_sorted_vec()
        .into_iter()
        .enumerate()
    {
        solutions += 1;
        println!("Solution {n}:");
        for (order, charges) in solution {
            println!("{order}");
            for charge in charges {
                println!("- {charge}");
            }
        }
        println!();
    }

    if solutions == 0 {
        println!("No exact solutions!");
    }

    Ok(())
}
