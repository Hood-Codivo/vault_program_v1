# Vault Program V1

Vault Program V1 is a Solana Anchor program that lets a user create a personal SOL vault, deposit lamports into it, withdraw lamports from it, and close the vault state when finished.

The project is built with Anchor and includes a Rust LiteSVM integration test that exercises the full initialize -> deposit -> withdraw -> close flow locally.

## Program Summary

Program ID:


```text
AuD9H9gzoj3vdETqjbZeotaaHnDDoQNB1XtuWB983Hsi
```

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/45e25e5e-d913-4052-ab59-4f4338bf68d5" />


Core instructions:

| Instruction  | Purpose                                                                                 |
| ------------ | --------------------------------------------------------------------------------------- |
| `initialize` | Creates the user's `VaultState` PDA and derives the vault PDA.                          |
| `deposit`    | Transfers lamports from the user into the vault PDA.                                    |
| `withdraw`   | Transfers lamports from the vault PDA back to the user.                                 |
| `close`      | Transfers the remaining vault lamports to the user and closes the `VaultState` account. |

## Account Model

The program uses two PDAs per user:

| Account       | Seeds                                    | Purpose                                       |
| ------------- | ---------------------------------------- | --------------------------------------------- |
| `vault_state` | `[b"state", user.key().as_ref()]`        | Stores the vault and state bumps.             |
| `vault`       | `[b"vault", vault_state.key().as_ref()]` | System account that holds deposited lamports. |

`VaultState` stores:

```rust
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}
```

## Project Structure

```text
.
|-- Anchor.toml
|-- Cargo.toml
|-- programs/
|   `-- vault_program_v1/
|       |-- Cargo.toml
|       |-- src/
|       |   |-- lib.rs
|       |   |-- state.rs
|       |   `-- instructions/
|       |       |-- initialize.rs
|       |       |-- deposit.rs
|       |       |-- withdraw.rs
|       |       `-- close.rs
|       `-- tests/
|           `-- test_initialize.rs
`-- target/
```

## Prerequisites

Install the usual Solana Anchor tooling:

- Rust
- Solana CLI
- Anchor CLI
- Yarn

The workspace is configured for `localnet` in `Anchor.toml`.

## Build

From the project root:

```bash
anchor build
```

This produces the deployable program binary at:

```text
target/deploy/vault_program_v1.so
```

The LiteSVM Rust test loads this file directly, so rebuild with `anchor build` after changing program logic.

## Test

Run the Rust integration test:

```bash
cargo test -p vault_program_v1 --test test_initialize
```

Or use the Anchor test script configured in `Anchor.toml`:

```bash
anchor test
```

The current test:

1. Creates a LiteSVM test bank.
2. Adds the compiled `vault_program_v1.so`.
3. Airdrops SOL to a payer.
4. Initializes the vault state.
5. Deposits 1 SOL into the vault.
6. Withdraws 0.5 SOL.
7. Closes the vault state.

## Development Notes

- `msg!` is for on-chain program logs. Use `println!` inside off-chain Rust tests.
- Deposit uses a normal system transfer from the user to the vault.
- Withdraw and close use signed CPI transfers from the vault PDA back to the user.
- The PDA seeds in instruction account constraints must match the seeds used by the test and initializer.
