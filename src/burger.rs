use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
enum Bun {
    #[display(fmt = "sesame")]
    Sesame,
    #[display(fmt = "brioche")]
    Brioche,
    #[display(fmt = "whole_wheat")]
    WholeWheat,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
enum Patty {
    #[display(fmt = "single")]
    Single,
    #[display(fmt = "double")]
    Double,
    #[display(fmt = "black_bean")]
    BlackBean,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
enum Cheese {
    #[display(fmt = "american")]
    American,
    #[display(fmt = "cheddar")]
    Cheddar,
    #[display(fmt = "pepperjack")]
    Pepperjack,
    #[display(fmt = "swiss")]
    Swiss,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
enum Topping {
    #[display(fmt = "lettuce")]
    Lettuce,
    #[display(fmt = "tomato")]
    Tomato,
    #[display(fmt = "grilled_onions")]
    GrilledOnions,
    #[display(fmt = "bacon")]
    Bacon,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
enum Sauce {
    #[display(fmt = "mustard")]
    Mustard,
    #[display(fmt = "ketchup")]
    Ketchup,
    #[display(fmt = "mayo")]
    Mayo,
    #[display(fmt = "barbecue")]
    Barbecue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Burger {
    bun: Bun,
    patty: Patty,
    cheese: Option<Cheese>,
    toppings: Vec<Topping>,
    sauces: Vec<Sauce>,
}

impl Burger {
    pub fn list_ingredients() -> HashMap<String, Vec<String>> {
        let mut ingredients = HashMap::new();

        ingredients.insert(
            "bun".to_string(),
            vec![
                Bun::Sesame.to_string(),
                Bun::Brioche.to_string(),
                Bun::WholeWheat.to_string(),
            ],
        );

        ingredients.insert(
            "patty".to_string(),
            vec![
                Patty::Single.to_string(),
                Patty::Double.to_string(),
                Patty::BlackBean.to_string(),
            ],
        );

        ingredients.insert(
            "cheese".to_string(),
            vec![
                Cheese::American.to_string(),
                Cheese::Cheddar.to_string(),
                Cheese::Pepperjack.to_string(),
                Cheese::Swiss.to_string(),
            ],
        );

        ingredients.insert(
            "toppings".to_string(),
            vec![
                Topping::Lettuce.to_string(),
                Topping::Tomato.to_string(),
                Topping::GrilledOnions.to_string(),
                Topping::Bacon.to_string(),
            ],
        );

        ingredients.insert(
            "sauces".to_string(),
            vec![
                Sauce::Mustard.to_string(),
                Sauce::Ketchup.to_string(),
                Sauce::Mayo.to_string(),
                Sauce::Barbecue.to_string(),
            ],
        );

        ingredients
    }
}
