use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
pub enum UserState {
    Active,
    Inactive,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
pub struct User {
    pub id: String,
    pub name: String,
    pub state: UserState,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "User[]")]
    pub type Users;
}

#[wasm_bindgen(js_name = "getUsers")]
pub fn get_users() -> Users {
    let users: Vec<User> = [
        User {
            id: String::from("586f89c3-f7d8-4371-9d2d-4c90ba9eacb9"),
            name: String::from("User 1"),
            state: UserState::Active,
        },
        User {
            id: String::from("fae6ae60-ee3a-4e02-a4e4-64b333fa2919"),
            name: String::from("User 2"),
            state: UserState::Inactive,
        },
    ]
    .to_vec();

    Users::from(serde_wasm_bindgen::to_value(&users).unwrap())
}
