use soroban_sdk::{contractimpl, log, BytesN, Env, Map};
mod game_engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}

const ENGINE_ID: &str = "engine";

pub struct LoggingEngine;
#[contractimpl]
impl LoggingEngine {
    pub fn wrap(env: Env, engine_id: BytesN<32>) {
        env.storage().set(&ENGINE_ID, &engine_id);
        log!(&env, "🗒️ logger engine taking notes");
    }

    fn engine_id(env: Env) -> BytesN<32> {
        env.storage().get(&ENGINE_ID).unwrap().unwrap()
    }
    fn get_engine(env: &Env) -> game_engine::Client {
        game_engine::Client::new(&env, &Self::engine_id(env.clone()))
    }

    /// wrapping interface implemention
    pub fn init(
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
        if !env.storage().has(&ENGINE_ID) {
            log!(&env, "Call 'wrap' first");
            panic!();
        }

        Self::get_engine(&env).init(
            &move_step,
            &laser_range,
            &seed,
            &view_range,
            &fuel,
            &asteroid_reward,
            &asteroid_density,
            &pod_density,
        );
    }
    pub fn p_turn(env: Env, direction: game_engine::Direction) -> Result<(), game_engine::Error> {
        Ok(Self::get_engine(&env).p_turn(&direction))
    }
    pub fn p_move(env: Env, times: Option<u32>) -> Result<(), game_engine::Error> {
        Ok(Self::get_engine(&env).p_move(&times))
    }
    pub fn p_shoot(env: Env) -> Result<(), game_engine::Error> {
        Ok(Self::get_engine(&env).p_shoot())
    }
    pub fn p_harvest(env: Env) -> Result<(), game_engine::Error> {
        Ok(Self::get_engine(&env).p_harvest())
    }
    pub fn p_upgrade(env: Env) -> Result<(), game_engine::Error> {
        Ok(Self::get_engine(&env).p_upgrade())
    }
    pub fn p_pos(env: Env) -> game_engine::Point {
        Self::get_engine(&env).p_pos()
    }
    pub fn p_dir(env: Env) -> game_engine::Direction {
        Self::get_engine(&env).p_dir()
    }
    pub fn p_points(env: Env) -> u32 {
        Self::get_engine(&env).p_points()
    }
    pub fn p_fuel(env: Env) -> u32 {
        Self::get_engine(&env).p_fuel()
    }
    pub fn get_map(env: Env) -> Map<game_engine::Point, game_engine::MapElement> {
        Self::get_engine(&env).get_map()
    }
}
