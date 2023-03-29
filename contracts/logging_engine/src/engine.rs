use soroban_sdk::{contractimpl, Env, Map};
mod game_engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}
use game_engine::Contract;

pub struct LoggingEngine;
#[contractimpl]
impl Contract for LoggingEngine {
    fn init(
        env: Env,
        move_step: u32,
        laser_range: u32,
        seed: u64,
        view_range: u32,
        fuel: (u32, u32, u32, u32),
        asteroid_reward: u32,
        asteroid_density: u32,
        pod_density: u32,
    ) {
        todo!("needs implementation")
    }
    fn p_turn(env: Env, direction: game_engine::Direction) -> Result<(), game_engine::Error> {
        todo!("needs implementation")
    }
    fn p_move(env: Env, times: Option<u32>) -> Result<(), game_engine::Error> {
        todo!("needs implementation")
    }
    fn p_shoot(env: Env) -> Result<(), game_engine::Error> {
        todo!("needs implementation")
    }
    fn p_harvest(env: Env) -> Result<(), game_engine::Error> {
        todo!("needs implementation")
    }
    fn p_upgrade(env: Env) -> Result<(), game_engine::Error> {
        todo!("needs implementation")
    }
    fn p_pos(env: Env) -> game_engine::Point {
        todo!("needs implementation")
    }
    fn p_dir(env: Env) -> game_engine::Direction {
        todo!("needs implementation")
    }
    fn p_points(env: Env) -> u32 {
        todo!("needs implementation")
    }
    fn p_fuel(env: Env) -> u32 {
        todo!("needs implementation")
    }
    fn get_map(env: Env) -> Map<game_engine::Point, game_engine::MapElement> {
        todo!("needs implementation")
    }
}
