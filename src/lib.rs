use std::{collections::HashMap, fs::File, io::Read};

use serde::{Deserialize, Serialize, __private::de::IdentifierDeserializer};
use serde_json::Result;

#[derive(Serialize, Deserialize, Clone)]
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
    pub fn get_ingredient(&self, name: &String) -> Option<&Ingredient> {
        self.ingredients.get(name)
    }
}
