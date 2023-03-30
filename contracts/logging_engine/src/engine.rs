use soroban_sdk::{contractimpl, log, panic_with_error, vec, BytesN, Env, Map, Vec};
use crate::types::{Action, ActionItem, ProxyError};

mod game_engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}

const ACTIONS: &str = "actions";
const ENGINE_ID: &str = "engine";

pub struct LoggingEngine;
#[contractimpl]
impl LoggingEngine {
    pub fn wrap(env: Env, engine_id: BytesN<32>) {
        env.storage().set(&ENGINE_ID, &engine_id);
        env.storage()
            .set::<&str, Vec<ActionItem>>(&ACTIONS, &vec![&env]);
        log!(&env, "🗒️ logger engine taking notes");
    }

    fn engine_id(env: Env) -> BytesN<32> {
        env.storage().get(&ENGINE_ID).unwrap().unwrap()
    }
    fn get_engine(env: &Env) -> game_engine::Client {
        game_engine::Client::new(&env, &Self::engine_id(env.clone()))
    }

    pub fn actions(env: Env) -> Vec<ActionItem> {
        env.storage().get(&ACTIONS).unwrap().unwrap()
    }
    fn log_action(env: &Env, action: &ActionItem) {
        let mut actions = Self::actions(env.clone());
        actions.push_back(*action);
        env.storage().set(&ACTIONS, &actions);
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
            panic_with_error!(&env, ProxyError::NotWrapped);
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
        if let Err(Ok(e)) = Self::get_engine(&env).try_p_turn(&direction) {
            return Err(e);
        }
        Self::log_action(&env, &ActionItem(Action::Turn, direction as u32));
        Ok(())
    }
    pub fn p_move(env: Env, times: Option<u32>) -> Result<(), game_engine::Error> {
        if let Err(Ok(e)) = Self::get_engine(&env).try_p_move(&times) {
            return Err(e);
        }
        Self::log_action(&env, &ActionItem(Action::Move, 1));
        Ok(())
    }
    pub fn p_shoot(env: Env) -> Result<(), game_engine::Error> {
        let p = Self::get_engine(&env).p_points();
        if let Err(Ok(e)) = Self::get_engine(&env).try_p_shoot() {
            return Err(e);
        }
        let hits = Self::get_engine(&env).p_points() - p;
        Self::log_action(&env, &ActionItem(Action::Shoot, hits));
        Ok(())
    }
    pub fn p_harvest(env: Env) -> Result<(), game_engine::Error> {
        if let Err(Ok(e)) = Self::get_engine(&env).try_p_harvest() {
            return Err(e);
        }
        Self::log_action(&env, &ActionItem(Action::Harvest, 1));
        Ok(())
    }
    pub fn p_upgrade(env: Env) -> Result<(), game_engine::Error> {
        if let Err(Ok(e)) = Self::get_engine(&env).try_p_upgrade() {
            return Err(e);
        }
        Self::log_action(&env, &ActionItem(Action::Upgrade, 1));
        Ok(())
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
