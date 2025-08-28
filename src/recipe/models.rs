#[derive(Debug, serde::Deserialize)]
pub struct MealsResponse {
    pub meals: Option<Vec<Meal>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MealListResponse {
    pub meals: Option<Vec<MealMin>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MealMin {
    #[serde(rename = "idMeal")]
    pub id: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Meal {
    #[serde(rename = "idMeal")]
    pub id: Option<String>,
    #[serde(rename = "strMeal")]
    pub name: String,
    #[serde(rename = "strInstructions")]
    pub instructions: String,
    #[serde(rename = "strCategory")]
    pub category: Option<String>,
    #[serde(rename = "strMealThumb")]
    pub thumbnail: Option<String>,
    #[serde(flatten)]
    pub(crate) extra: std::collections::HashMap<String, Option<String>>,
}

impl Meal {
    pub(crate) fn get_ingredients(&self) -> Vec<(String, String)> {
        let mut ingredients = Vec::new();
        for i in 1..=20 {
            let ingredient_key = format!("strIngredient{}", i);
            let measure_key = format!("strMeasure{}", i);
            if let (Some(Some(ingredient)), Some(Some(measure))) = (self.extra.get(&ingredient_key), self.extra.get(&measure_key)) {
                if !ingredient.trim().is_empty() {
                    ingredients.push((ingredient.clone(), measure.clone()));
                } else { break; }
            } else { break; }
        }
        ingredients
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CategoriesResponse {
    pub meals: Vec<Category>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Category {
    #[serde(rename = "strCategory")]
    pub name: String,
}

#[derive(Debug, Clone, poise::ChoiceParameter)]
pub enum MealCategory {
    #[name = "Beef"] Beef,
    #[name = "Chicken"] Chicken,
    #[name = "Dessert"] Dessert,
    #[name = "Lamb"] Lamb,
    #[name = "Miscellaneous"] Miscellaneous,
    #[name = "Pasta"] Pasta,
    #[name = "Pork"] Pork,
    #[name = "Seafood"] Seafood,
    #[name = "Side"] Side,
    #[name = "Starter"] Starter,
    #[name = "Vegan"] Vegan,
    #[name = "Vegetarian"] Vegetarian,
    #[name = "Breakfast"] Breakfast,
    #[name = "Goat"] Goat,
}