#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

/// Resep yang disimpan on-chain: judul, bahan, dan langkah memasak.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Recipe {
    pub id: u64,
    pub title: String,
    pub ingredients: String,
    pub instructions: String,
}

const RECIPE_DATA: Symbol = symbol_short!("RCP_DATA");

#[contract]
pub struct RecipesContract;

#[contractimpl]
impl RecipesContract {
    /// Mengambil semua resep dari penyimpanan kontrak.
    pub fn get_recipes(env: Env) -> Vec<Recipe> {
        env.storage()
            .instance()
            .get(&RECIPE_DATA)
            .unwrap_or_else(|| Vec::new(&env))
    }

    /// Menambah resep baru; ID dihasilkan secara acak aman di dalam kontrak.
    pub fn create_recipe(
        env: Env,
        title: String,
        ingredients: String,
        instructions: String,
    ) -> String {
        let mut recipes: Vec<Recipe> = env
            .storage()
            .instance()
            .get(&RECIPE_DATA)
            .unwrap_or_else(|| Vec::new(&env));

        let recipe = Recipe {
            id: env.prng().gen::<u64>(),
            title,
            ingredients,
            instructions,
        };
        recipes.push_back(recipe);
        env.storage().instance().set(&RECIPE_DATA, &recipes);

        String::from_str(&env, "Resep berhasil ditambahkan")
    }

    /// Menghapus resep berdasarkan ID.
    pub fn delete_recipe(env: Env, id: u64) -> String {
        let mut recipes: Vec<Recipe> = env
            .storage()
            .instance()
            .get(&RECIPE_DATA)
            .unwrap_or_else(|| Vec::new(&env));

        for i in 0..recipes.len() {
            if recipes.get(i).unwrap().id == id {
                recipes.remove(i);
                env.storage().instance().set(&RECIPE_DATA, &recipes);
                return String::from_str(&env, "Resep berhasil dihapus");
            }
        }

        String::from_str(&env, "Resep tidak ditemukan")
    }
}

mod test;
