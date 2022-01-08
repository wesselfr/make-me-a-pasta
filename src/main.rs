use mmp::*;

fn main() {
    let mut collection: IngredientCollection = IngredientCollection::new();
    collection.load();
    let mut try_count = 0;

    // - Pasta
    let pasta = collection.get_random_ingredient(IngredientType::Pasta);
    
    // - Sauce -> Based on pasta
    //let sauce = collection.get_random_ingredient(IngredientType::Sauce);
    let sauce = collection.get_random_ingredient(IngredientType::Sauce);

    // - Meat -> Based on Sauce (or no meat for vegetarians)
    let mut meat = collection.get_random_matching_ingredient(IngredientType::Meat, [sauce].to_vec());
    while meat.is_none() && try_count < 3 {
        //println!("FAIL MATCH IN MEAT");
        meat = collection.get_random_matching_ingredient(IngredientType::Meat, [sauce].to_vec());
        try_count += 1;
    }

    // - (Some) Vegtables -> Based on Sauce and Pasta
    //let vegtable = collection.get_random_ingredient(IngredientType::Vegtable);
    let mut vegtable = collection.get_random_matching_ingredient(IngredientType::Vegtable, [sauce,pasta].to_vec());
    try_count = 0;
    while vegtable.is_none() && try_count < 3 {
        //println!("FAIL MATCH IN VEGTABLE");
        vegtable = collection.get_random_matching_ingredient(IngredientType::Vegtable, [sauce,pasta].to_vec());
        try_count += 1;
    }

    // - Cheese -> Based on Sauce and Pasta
    let mut cheese = collection.get_random_matching_ingredient(IngredientType::Cheese, [sauce,pasta].to_vec());
    try_count = 0;
    while cheese.is_none() && try_count < 3 {
        //println!("FAIL MATCH IN CHEESE");
        cheese = collection.get_random_matching_ingredient(IngredientType::Cheese, [sauce,pasta].to_vec());
        try_count += 1;
    }

    let mut recipe = Recipe::new();
    recipe.ingredients.push(pasta.clone());
    recipe.ingredients.push(sauce.clone());
    recipe.ingredients.push(meat.expect("Meat was expected..").clone());
    recipe.ingredients.push(vegtable.expect("Vegtable was expected..").clone());
    recipe.ingredients.push(cheese.expect("Cheese was expected..").clone());

    println!("Generated recipe for: {} {} with {}", recipe.ingredients[0].name, recipe.ingredients[1].name,recipe.ingredients[3].name);

    println!("");
    recipe.print_ingredients();
    println!("");

    println!("Steps:");
    println!("1) Bring a large pot of salted water to a boil and cook the {} 'al dente' in roughly {} minutes.", recipe.ingredients[0].name, recipe.ingredients[0].time);
    println!("2) Meanwhile heat some olive oil in the pan and cook the {} tender in {} minutes.", recipe.ingredients[2].name, recipe.ingredients[2].time);
    println!("3) Add the {} and cook for {} minutes.",recipe.ingredients[3].name, recipe.ingredients[3].time);
    println!("4) Next add the {} sauce, and heat for {} minutes on medium-high heat.", recipe.ingredients[1].name, recipe.ingredients[1].time);
    println!("5) Finally serve the pasta and add some {} on top for garnish.", recipe.ingredients[4].name);
}