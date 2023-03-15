use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use tokio;

#[tokio::test]
async fn list_ingredients() {
    let scheme = std::env::var("API_SCHEME").expect("api scheme");
    let host = std::env::var("API_HOST").expect("api host");

    let list_ingredients_res = Client::new()
        .get(format!("{}://{}/ingredients", scheme, host))
        .send()
        .await
        .expect("list ingredients res");

    assert_eq!(list_ingredients_res.status(), 200);

    let list_ingredients_body = list_ingredients_res
        .json::<HashMap<String, Vec<String>>>()
        .await
        .expect("list ingredients body");

    // TODO-- add helper function to assert valid options, ie 'isValidBun(option: &str) -> bool'
    for (ingredient_category, options) in list_ingredients_body {
        if ingredient_category == "bun" {
            for option in options {
                if option != "sesame" && option != "brioche" && option != "whole_wheat" {
                    assert!(false, "{} is not a valid bun option", option);
                }
            }
        } else if ingredient_category == "patty" {
            for option in options {
                if option != "single" && option != "double" && option != "black_bean" {
                    assert!(false, "{} is not a valid patty option", option);
                }
            }
        } else if ingredient_category == "cheese" {
            for option in options {
                if option != "american"
                    && option != "cheddar"
                    && option != "pepperjack"
                    && option != "swiss"
                {
                    assert!(false, "{} is not a valid cheese option", option);
                }
            }
        } else if ingredient_category == "toppings" {
            for option in options {
                if option != "lettuce"
                    && option != "tomato"
                    && option != "grilled_onions"
                    && option != "bacon"
                {
                    assert!(false, "{} is not a valid topping option", option);
                }
            }
        } else if ingredient_category == "sauces" {
            for option in options {
                if option != "mustard"
                    && option != "ketchup"
                    && option != "mayo"
                    && option != "barbecue"
                {
                    assert!(false, "{} is not a valid sauce option", option);
                }
            }
        } else {
            assert!(
                false,
                "{} is not a valid ingredient category",
                ingredient_category
            )
        }
    }
}

#[tokio::test]
async fn create_burger() {
    let scheme = std::env::var("API_SCHEME").expect("api scheme");
    let host = std::env::var("API_HOST").expect("api host");

    let create_burger_res = Client::new()
        .post(format!("{}://{}/burgers", scheme, host))
        .send()
        .await
        .expect("create burger res");

    assert_eq!(create_burger_res.status(), 201);

    let create_burger_body = create_burger_res
        .json::<Value>()
        .await
        .expect("create burger body");

    assert!(&create_burger_body["bun"].is_string());
    assert!(&create_burger_body["patty"].is_string());
    // cheese is optional
    assert!(&create_burger_body["toppings"].is_array());
    assert!(&create_burger_body["sauces"].is_array());

    // TODO-- add helper function to assert valid options, ie 'isValidBun(option: &str) -> bool'
}
