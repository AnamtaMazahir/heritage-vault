# Heritage Vault (Time-Locked Smart Contract)

### Summary
The Heritage Vault is a secure, time-based savings contract built in Rust. It allows a user to lock SOL away for a specific period, acting as a "smart" commitment device.

### How it Works
1. **Init**: User creates a vault with a Unix timestamp.
2. **Deposit**: User moves funds into the vault account.
3. **Withdraw**: The contract verifies the owner's signature AND checks if the current network time is >= the unlock date.



### Safety Features
- **Ownership Verification**: Uses Anchor's `require_keys_eq` to prevent unauthorized drain.
- **On-chain Clock**: Uses the `Clock` sysvar for immutable time verification.
- **Explicit Errors**: Custom error codes for better UX.
