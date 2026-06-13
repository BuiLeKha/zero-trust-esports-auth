#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    Address, BytesN, Env,
};

// ---------------------------------------------------------------------------
// Storage Keys
// ---------------------------------------------------------------------------

/// Keys for instance storage (admin) and persistent storage (tickets, HWIDs).
#[contracttype]
pub enum DataKey {
    /// Stores the admin Address in instance storage.
    Admin,
    /// Stores a bool ticket flag per player Address in persistent storage.
    Ticket(Address),
    /// Stores a 32-byte HWID hash per player Address in persistent storage.
    Hwid(Address),
}

// ---------------------------------------------------------------------------
// Custom Error Enum
// ---------------------------------------------------------------------------

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    /// `init()` was already called — admin is already set.
    AlreadyInitialized = 1,
    /// Caller is not the designated admin.
    NotAuthorized = 2,
    /// Player does not hold a valid ticket.
    NoTicket = 3,
    /// Player has already bound a HWID — cannot re-bind.
    HwidAlreadyBound = 4,
}

// ---------------------------------------------------------------------------
// Contract Definition
// ---------------------------------------------------------------------------

#[contract]
pub struct ZeroTrustEsportsAuth;

#[contractimpl]
impl ZeroTrustEsportsAuth {

    // -----------------------------------------------------------------------
    // 1. init — deploy the contract and designate the admin
    // -----------------------------------------------------------------------

    /// Sets the admin address exactly once. Reverts with `AlreadyInitialized`
    /// if called again. The admin must authorise this call so that only the
    /// real deployer can become admin.
    ///
    /// # Arguments
    /// * `admin` – the Address that will have elevated privileges.
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        // Guard: prevent re-initialisation
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        // The proposed admin must sign the transaction.
        admin.require_auth();

        // Persist the admin in instance storage (shared across all invocations).
        env.storage().instance().set(&DataKey::Admin, &admin);

        Ok(())
    }

    // -----------------------------------------------------------------------
    // 2. issue_ticket — admin grants a ticket to a player
    // -----------------------------------------------------------------------

    /// Allows the admin to issue a participation ticket to `player`.
    /// Reverts with `NotAuthorized` if the caller is not the stored admin.
    ///
    /// # Arguments
    /// * `admin`  – must match the stored admin and must sign the transaction.
    /// * `player` – the Address receiving the ticket.
    pub fn issue_ticket(env: Env, admin: Address, player: Address) -> Result<(), Error> {
        // Fetch the stored admin; reuse the same DataKey enum.
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap(); // Safe: init() must have been called first.

        // The caller-supplied admin must match the stored one.
        if admin != stored_admin {
            return Err(Error::NotAuthorized);
        }

        // Require a valid signature from the admin account.
        admin.require_auth();

        // Write the ticket flag into persistent storage (survives ledger archival
        // when TTL is extended appropriately in production).
        env.storage()
            .persistent()
            .set(&DataKey::Ticket(player), &true);

        Ok(())
    }

    // -----------------------------------------------------------------------
    // 3. bind_hwid — player registers their machine's HWID hash (once only)
    // -----------------------------------------------------------------------

    /// Binds a 32-byte HWID hash to the calling player's address.
    /// Can only be called once per player; subsequent calls return
    /// `HwidAlreadyBound`. The player must sign the transaction to prove
    /// ownership of the address.
    ///
    /// # Arguments
    /// * `player`    – the player Address (must sign).
    /// * `hwid_hash` – SHA-256 (or equivalent) hash of the player's hardware ID.
    pub fn bind_hwid(
        env: Env,
        player: Address,
        hwid_hash: BytesN<32>,
    ) -> Result<(), Error> {
        // Ensure the player has an issued ticket before binding.
        let has_ticket: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Ticket(player.clone()))
            .unwrap_or(false);

        if !has_ticket {
            return Err(Error::NoTicket);
        }

        // Guard: each player may bind their HWID exactly once.
        if env
            .storage()
            .persistent()
            .has(&DataKey::Hwid(player.clone()))
        {
            return Err(Error::HwidAlreadyBound);
        }

        // The player must authorise this registration.
        player.require_auth();

        // Store the HWID hash permanently against the player's address.
        env.storage()
            .persistent()
            .set(&DataKey::Hwid(player), &hwid_hash);

        Ok(())
    }

    // -----------------------------------------------------------------------
    // 4. verify_login — read-only login check
    // -----------------------------------------------------------------------

    /// Returns `true` if and only if:
    ///   (a) the player holds a valid ticket, AND
    ///   (b) the supplied `hwid_hash` matches the one registered via `bind_hwid`.
    ///
    /// This function is intentionally read-only (no state mutation) so it can
    /// be called for free (simulation / preflight) without a transaction fee.
    ///
    /// # Arguments
    /// * `player`    – the Address to verify.
    /// * `hwid_hash` – the HWID hash presented at login time.
    pub fn verify_login(
        env: Env,
        player: Address,
        hwid_hash: BytesN<32>,
    ) -> bool {
        // Check 1 – does the player hold a ticket?
        let has_ticket: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Ticket(player.clone()))
            .unwrap_or(false);

        if !has_ticket {
            return false;
        }

        // Check 2 – does the stored HWID hash match the presented one?
        let stored_hwid: Option<BytesN<32>> = env
            .storage()
            .persistent()
            .get(&DataKey::Hwid(player));

        match stored_hwid {
            Some(stored) => stored == hwid_hash,
            None => false, // HWID has not been bound yet → deny
        }
    }
}

// ---------------------------------------------------------------------------
// Unit Tests (runs in the Soroban test environment, not on-chain)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, ZeroTrustEsportsAuthClient<'static>, Address, Address) {
        let env = Env::default();
        env.mock_all_auths(); // auto-approve all require_auth() calls in tests

        let contract_id = env.register(ZeroTrustEsportsAuth, ());
        let client = ZeroTrustEsportsAuthClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let player = Address::generate(&env);

        (env, client, admin, player)
    }

    #[test]
    fn full_happy_path() {
        let (_env, client, admin, player) = setup();

        // 1. Init
        client.init(&admin);

        // 2. Issue ticket
        client.issue_ticket(&admin, &player);

        // 3. Bind HWID
        let hwid = BytesN::from_array(&_env, &[0xabu8; 32]);
        client.bind_hwid(&player, &hwid);

        // 4. Verify login — must return true
        assert!(client.verify_login(&player, &hwid));
    }

    #[test]
    fn wrong_hwid_returns_false() {
        let (_env, client, admin, player) = setup();
        client.init(&admin);
        client.issue_ticket(&admin, &player);

        let correct_hwid = BytesN::from_array(&_env, &[0x01u8; 32]);
        let wrong_hwid = BytesN::from_array(&_env, &[0x02u8; 32]);
        client.bind_hwid(&player, &correct_hwid);

        assert!(!client.verify_login(&player, &wrong_hwid));
    }

    #[test]
    fn double_init_fails() {
        let (_env, client, admin, _player) = setup();
        client.init(&admin);
        let result = client.try_init(&admin);
        assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
    }

    #[test]
    fn double_bind_fails() {
        let (_env, client, admin, player) = setup();
        client.init(&admin);
        client.issue_ticket(&admin, &player);

        let hwid = BytesN::from_array(&_env, &[0xffu8; 32]);
        client.bind_hwid(&player, &hwid);

        let result = client.try_bind_hwid(&player, &hwid);
        assert_eq!(result, Err(Ok(Error::HwidAlreadyBound)));
    }

    #[test]
    fn no_ticket_bind_fails() {
        let (_env, client, admin, player) = setup();
        client.init(&admin);

        // Player has no ticket — binding should fail.
        let hwid = BytesN::from_array(&_env, &[0x11u8; 32]);
        let result = client.try_bind_hwid(&player, &hwid);
        assert_eq!(result, Err(Ok(Error::NoTicket)));
    }

    #[test]
    fn unauthorized_ticket_issue_fails() {
        let (_env, client, admin, player) = setup();
        client.init(&admin);

        // Use a random address as fake admin.
        let fake_admin = Address::generate(&_env);
        let result = client.try_issue_ticket(&fake_admin, &player);
        assert_eq!(result, Err(Ok(Error::NotAuthorized)));
    }
}