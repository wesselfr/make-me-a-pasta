use std::{collections::HashMap, fs::File, io::Read};

use serde::{Deserialize, Serialize, __private::de::IdentifierDeserializer};
use serde_json::Result;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum IngredientType {
    Pasta,
    Vegtable,
    Cheese,
    Meat,
    Sauce,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ingredient {
    pub name: String,
    pub weights: Vec<u32>,
    pub ingredient_type: IngredientType,

    pub matching_pasta: Vec<String>,
    pub matching_vegtables: Vec<String>,
    pub matching_cheese: Vec<String>,
    pub matching_meat: Vec<String>,
    pub matching_sauce: Vec<String>,
}

impl Ingredient {
    pub fn new(name: String, ingredient_type: IngredientType) -> Ingredient {
        Ingredient {
            name: name,
            weights: [0, 0, 0].to_vec(),
            ingredient_type: ingredient_type,
            matching_pasta: Vec::new(),
            matching_vegtables: Vec::new(),
            matching_cheese: Vec::new(),
            matching_meat: Vec::new(),
            matching_sauce: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IngredientCollection {
    pub ingredients: HashMap<String, Ingredient>,
}

impl IngredientCollection {
    pub fn new() -> IngredientCollection {
        IngredientCollection {
            ingredients: HashMap::new(),
        }
    }
    pub fn load(&mut self) {
        let file = File::open("ingredients.json");

        if file.is_ok() {
            let mut contents = String::new();
            file.unwrap().read_to_string(&mut contents).unwrap();

            self.load_ingredients(&contents);
        }
    }
    fn load_ingredients(&mut self, data: &str) -> Result<()> {
        let ingredients: IngredientCollection = serde_json::from_str(data)?;

        self.ingredients = ingredients.ingredients;
        Ok(())
    }
    pub fn get_ingredient(&self, name: &str) -> Option<&Ingredient> {
        self.ingredients.get(name)
    }
    pub fn get_random_ingredient(&self, ingredientType: IngredientType) {}
    pub fn is_matching_ingredient(&self, ingredient: &str, other_ingredient: &str) -> bool {
        let contained_ingredient = self.get_ingredient(ingredient).unwrap();
        let possible_ingredient = self.ingredients.get(other_ingredient).unwrap();

        match possible_ingredient.ingredient_type {
            IngredientType::Pasta => {
                return possible_ingredient.matching_pasta.len() == 0
                    || possible_ingredient
                        .matching_pasta
                        .contains(&contained_ingredient.name)
            }
            IngredientType::Vegtable => {
                return possible_ingredient.matching_vegtables.len() == 0
                    || possible_ingredient
                        .matching_vegtables
                        .contains(&contained_ingredient.name)
            }
            IngredientType::Cheese => {
                return possible_ingredient.matching_cheese.len() == 0
                    || possible_ingredient
                        .matching_cheese
                        .contains(&contained_ingredient.name)
            }
            IngredientType::Meat => {
                return possible_ingredient.matching_meat.len() == 0
                    || possible_ingredient
                        .matching_meat
                        .contains(&contained_ingredient.name)
            }
            IngredientType::Sauce => {
                return possible_ingredient.matching_sauce.len() == 0
                    || possible_ingredient
                        .matching_sauce
                        .contains(&contained_ingredient.name)
            }
        }
    }
    pub fn get_ingredient_of_type(&self, ingredient_type: IngredientType) -> Vec<&Ingredient> {
        let mut v = Vec::new();

        for item in &self.ingredients {
            if item.1.ingredient_type == ingredient_type {
                v.push(item.1);
            }
        }

        v
    }
}

pub struct Recipe {
    pub ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn new() -> Recipe {
        Recipe {
            ingredients: Vec::new(),
        }
    }
    pub fn print_ingredients(&self) {
        println!("Ingredients: ");
        for item in &self.ingredients {
            println!("{} grams of {}", item.weights[0], item.name);
        }
    }
}
