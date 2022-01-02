use std::{fs::File, io::Write};

use mmp::*;

fn main() {
    let mut collection: IngredientCollection = IngredientCollection::new();
    collection.load();

    // Print all stored ingredients
    for item in &collection.ingredients{
        println!("Ingredient: {}", item.1.name);
    }

    println!("Hello, world!");
}
