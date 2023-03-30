use soroban_sdk::{contracterror, contracttype};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum ProxyError {
    UnknownErr = 10,
    NotWrapped = 11,
}

#[contracttype]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Action {
    Upgrade,
    Shoot,
    Harvest,
    Turn,
    Move,
}

#[contracttype]
#[derive(Copy, Clone)]
pub struct ActionItem(pub Action, pub u32);
