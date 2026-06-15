
# Project Title
ZeroTrust Esports Auth

## Project Description
ZeroTrust Esports Auth is a decentralized smart contract built on Soroban (Stellar blockchain) designed to manage access and authenticate players for Esports tournaments. Utilizing a Zero Trust security model, the platform ensures fair play and prevents account sharing by requiring players to hold an admin-issued ticket and bind their account to a specific hardware device using a cryptographic HWID hash.

## Project Vision
The vision of ZeroTrust Esports Auth is to provide Esports organizers with a secure, tamper-proof, and decentralized authentication system. By eliminating centralized points of failure and tying access strictly to verified hardware and on-chain tickets, we aim to foster a fair, transparent, and cheat-free competitive environment for all participants.

## Key Features
* **Admin Initialization:** Secure, one-time setup to designate the contract administrator with elevated privileges.
* **Ticket Issuance:** Admin-controlled distribution of participation tickets to verified player wallet addresses.
* **Zero-Trust Verification:** Read-only, gas-free login verification requiring both a valid ticket flag and a strictly matching HWID hash.
* **Immutable HWID Binding:** Players bind their unique 32-byte Hardware ID (HWID) hash to their wallet address (enforced as a one-time only action to prevent device swapping).
* **Access Control:** Strict authorization checks (`require_auth`) ensuring only the admin can issue tickets and only the respective player can bind their device.

## Future Scope
* **Tournament Tiers:** Support for multiple ticket types (e.g., VIP, Standard, Qualifier).
* **HWID Appeal Mechanism:** Implement an admin-controlled process to safely reset a player's HWID in case of legitimate hardware failure.
* **Anti-cheat Integration:** Direct off-chain integration tools and SDKs for popular anti-cheat engines (e.g., Vanguard, Easy Anti-Cheat).
* **Automated Ticket Sales:** Allow players to purchase entry tickets directly using Soroban tokens (e.g., USDC or XLM) instead of manual admin issuance.
* **Revocation System:** Allow admins to revoke tickets dynamically if cheating or policy violations are detected.

## Contract Details
* **Contract ID:** CCSVN3KN47YKDXPPSVWMF4MN5XBIGH2GZUC5DWWUW4OLTMHVALZNUPY4
* **Network:** Testnet
* **Block Explorer:** https://stellar.expert/explorer/testnet/tx/0d642d79ee3f29b809a4183a821df13934cbca5cf71c32ba56391908dfda26e2

<img width="1874" height="837" alt="stellar" src="https://github.com/user-attachments/assets/3816a987-0f79-48b3-8d0c-29b82874b8a4" />

---

## Usage Instructions

1. **Set Admin (`init`):** Deploy the contract and call the initialization function to assign the admin address. This can only be done once.
2. **Issue Ticket (`issue_ticket`):** The admin calls this function to grant a participation ticket to a specific player's address.
3. **Bind HWID (`bind_hwid`):** The player signs a transaction to submit their 32-byte HWID hash, permanently linking their local device to their wallet address.
4. **Verify Login (`verify_login`):** Game launchers or clients query this read-only function, passing the player's address and current HWID hash. Access is only granted if it returns `true`.

## Technology Stack
* **Rust and Soroban SDK** for secure smart contract development.
* **Stellar Blockchain** for decentralized, immutable state management.
* **Cryptographic Hashing (SHA-256)** for secure subscription enforcement.

## Contribution
Community contributions are welcomed from blockchain developers and Esports platform experts. Fork the repository and submit pull requests to assist in further development.

## License
This project is licensed under the MIT License.
