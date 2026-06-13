#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    Address, BytesN, Env,
};

#[contracttype]
pub enum DataKey {
    Admin,
    Ticket(Address),
    Hwid(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotAuthorized      = 2,
    NoTicket           = 3,
    HwidAlreadyBound   = 4,
}

#[contract]
pub struct ZeroTrustEsportsAuth;

#[contractimpl]
impl ZeroTrustEsportsAuth {

    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        let store = env.storage().instance();
        if store.has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        store.set(&DataKey::Admin, &admin);
        Ok(())
    }

    pub fn issue_ticket(env: Env, admin: Address, player: Address) -> Result<(), Error> {
        let stored: Address = env.storage().instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotAuthorized)?;
        if admin != stored { return Err(Error::NotAuthorized); }
        admin.require_auth();
        env.storage().persistent().set(&DataKey::Ticket(player), &true);
        Ok(())
    }

    pub fn bind_hwid(env: Env, player: Address, hwid_hash: BytesN<32>) -> Result<(), Error> {
        player.require_auth();
        let has_ticket: bool = env.storage().persistent()
            .get(&DataKey::Ticket(player.clone()))
            .unwrap_or(false);
        if !has_ticket { return Err(Error::NoTicket); }
        if env.storage().persistent().has(&DataKey::Hwid(player.clone())) {
            return Err(Error::HwidAlreadyBound);
        }
        env.storage().persistent().set(&DataKey::Hwid(player), &hwid_hash);
        Ok(())
    }

    pub fn verify_login(env: Env, player: Address, hwid_hash: BytesN<32>) -> bool {
        let has_ticket: bool = env.storage().persistent()
            .get(&DataKey::Ticket(player.clone()))
            .unwrap_or(false);
        if !has_ticket { return false; }
        env.storage().persistent()
            .get::<DataKey, BytesN<32>>(&DataKey::Hwid(player))
            .map(|stored| stored == hwid_hash)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_happy_path() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(ZeroTrustEsportsAuth, ());
        let client = ZeroTrustEsportsAuthClient::new(&env, &id);

        let admin  = Address::generate(&env);
        let player = Address::generate(&env);
        let hwid   = BytesN::from_array(&env, &[1u8; 32]);

        client.init(&admin);
        client.issue_ticket(&admin, &player);
        client.bind_hwid(&player, &hwid);

        assert!(client.verify_login(&player, &hwid));
        assert!(!client.verify_login(&player, &BytesN::from_array(&env, &[9u8; 32])));
    }
}