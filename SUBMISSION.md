# Junior Dev Explanation: How I Built This

* **The State**: We use a `struct` called `Vault` to store data on-chain. It’s like a tiny row in a database that keeps track of the owner and the "Release Date."
* **The Guard**: I used `require!` macros. Think of these as bouncers at a club. If your name isn't on the list (Ownership) or it's not the right time (Clock), you're not getting in.
* **The Clock**: Blockchains have their own time. I pulled the current time using `Clock::get()`. This is safer than using a user's local computer time, which can be faked.
* **CPI**: To move money, I used a "Cross-Program Invocation." This is basically my contract calling the main Solana System Program to say, "Hey, move these funds for me."
* **Safety First**: I used a custom `error_code` enum so that if a transaction fails, the user knows exactly why (e.g., "Patience: The unlock date has not arrived").
