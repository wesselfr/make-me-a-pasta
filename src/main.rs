use mmp::*;

fn main() {
    let mut collection: IngredientCollection = IngredientCollection::new();
    collection.load();

    println!("Possible ingredients");
    // Print all stored ingredients
    for item in &collection.ingredients {
        println!("Ingredient: {}", item.1.name);
    }

    println!("NEW RECIPE");
    let spaghetti = collection.get_ingredient("Spaghetti");
    let carbonara = collection.get_ingredient("Carbonara");

    let mut recipe = Recipe::new();
    recipe
        .ingredients
        .push(collection.get_ingredient("Spaghetti").unwrap().clone());
    recipe
        .ingredients
        .push(collection.get_ingredient("Carbonara").unwrap().clone());

    recipe.print_ingredients();

    println!(
        "Does match: {}",
        collection.is_matching_ingredient("Spaghetti", "Carbonara")
    );
    println!(
        "Does match: {}",
        collection.is_matching_ingredient("Carbonara", "Spaghetti")
    );

    println!("Collection has the following cheeses:");
    let cheeses = collection.get_ingredient_of_type(IngredientType::Cheese);
    for cheese in cheeses {
        println!("{}", cheese.name);
    }

    println!("Hello, world!");
}
