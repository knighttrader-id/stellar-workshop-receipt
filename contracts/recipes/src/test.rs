#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn create_get_and_delete_recipe() {
    let env = Env::default();
    let contract_id = env.register(RecipesContract, ());
    let client = RecipesContractClient::new(&env, &contract_id);

    let title = String::from_str(&env, "Nasi Goreng");
    let ingredients = String::from_str(&env, "Nasi dingin, telur, kecap manis, bawang");
    let instructions = String::from_str(&env, "Tumis bawang, masukkan telur, nasi, kecap.");

    let msg = client.create_recipe(&title, &ingredients, &instructions);
    assert_eq!(msg, String::from_str(&env, "Resep berhasil ditambahkan"));

    let recipes = client.get_recipes();
    assert_eq!(recipes.len(), 1);
    let r = recipes.get(0).unwrap();
    assert_eq!(r.title, title);
    assert_eq!(r.ingredients, ingredients);
    assert_eq!(r.instructions, instructions);
    let id = r.id;

    let del = client.delete_recipe(&id);
    assert_eq!(del, String::from_str(&env, "Resep berhasil dihapus"));
    assert_eq!(client.get_recipes().len(), 0);
}

#[test]
fn delete_unknown_returns_message() {
    let env = Env::default();
    let contract_id = env.register(RecipesContract, ());
    let client = RecipesContractClient::new(&env, &contract_id);

    let r = client.delete_recipe(&999_u64);
    assert_eq!(r, String::from_str(&env, "Resep tidak ditemukan"));
}
