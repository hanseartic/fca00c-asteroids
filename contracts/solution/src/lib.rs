#![no_std]

use engine::{Client as GameEngine, Direction};
use soroban_sdk::{contractimpl, BytesN, Env};

pub struct Solution;

mod engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}

mod test;

#[contractimpl]
impl Solution {
    pub fn solve(env: Env, engine_id: BytesN<32>) {
        let engine = GameEngine::new(&env, &engine_id);

        // YOUR CODE START

        engine.p_shoot();
        engine.p_move(&None);
        engine.p_move(&Some(2));
        engine.p_turn(&Direction::UpLeft);
        engine.p_turn(&Direction::Left);
        engine.p_move(&Some(1));
        engine.p_turn(&Direction::Down);
        // YOUR CODE END
    }
}
