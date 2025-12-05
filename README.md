

#  Message Storage Program (Solana + Anchor)

A minimal Solana smart contract that allows users to:

* Initialize an on-chain account with a string message
* Update the stored message
* Read the stored message on the client side

This program is built using **Rust + Anchor framework**.

---

##  Prerequisites

Before working with this project, install:

| Tool       | Version Required                |
| ---------- | ------------------------------- |
| Rust       | `>= 1.75`                       |
| Solana CLI | `1.18.x` (localnet recommended) |
| Anchor CLI | `0.32.x`                        |
| Node.js    | `>= 18`                         |
| Yarn       | `>= 1`                          |

Verify installation:

```bash
rustc --version
solana --version
anchor --version
node --version
```

---

##  Clone Repository

```bash
git clone https://github.com/ItzDhruv/Message_program
cd message-program
```

---

##  Configure Solana Localnet

Set Solana to local network:

```bash
solana config set --url http://127.0.0.1:8899
```

Create wallet if missing:

```bash
solana-keygen new --outfile ~/.config/solana/id.json
```

---

##  Start Local Validator

In **Terminal 1**:

```bash
solana-test-validator
```

Keep this running.

Airdrop SOL to wallet:

```bash
solana airdrop 10
```

Check balance:

```bash
solana balance
```

---

##  Build & Deploy Program

In **Terminal 2**:

```bash
anchor build
anchor deploy
```

If deployment succeeds, you will see:

```
Deploy success
```

---

##  Run Test Suite

Program already deployed & validator running → run:

```bash
anchor test --skip-local-validator
```

Expected output:

```
Stored Message: Hello, Solana!
Updated Message: Updated message from test!
2 passing
```

---

##  Project Structure

```
message-program/
│  Anchor.toml
│  Cargo.toml
│  README.md
│
├─ programs/
│   └─ message-program/
│         └─ src/lib.rs   ← on-chain smart contract
│
├─ tests/
│   └─ message-program.ts ← test cases using TypeScript
│
└─ target/                ← compiled artifacts (auto-generated)
```

---

##  How the Program Works

### On-Chain Instructions

| Instruction                          | Description                              |
| ------------------------------------ | ---------------------------------------- |
| `initialize(message: String)`        | Creates account + stores initial message |
| `updateMessage(new_message: String)` | Updates existing message                 |

### Stored Account Data

```rust
pub struct MessageAccount {
    pub authority: Pubkey,
    pub message: String,
}
```

* Only the wallet that initialized the account is authorized to update it.

### Reading Message

There is **no "read" instruction**.
Clients simply fetch account data:

```ts
const account = await program.account.messageAccount.fetch(messagePubkey);
console.log(account.message);
```

---

##  Deploying to Devnet (optional)

```bash
solana config set --url https://api.devnet.solana.com
solana airdrop 1   # might require faucet link
anchor deploy
```

---

##  Troubleshooting

| Issue                          | Fix                                                                    |
| ------------------------------ | ---------------------------------------------------------------------- |
| `solana: command not found`    | Add Solana to PATH                                                     |
| `airdop failed`                | Validator not running → run `solana-test-validator`                    |
| `DeclaredProgramIdMismatch`    | Program ID mismatch → update `declare_id!` and `Anchor.toml`           |
| `rpc port 8899 already in use` | A validator already running → use `anchor test --skip-local-validator` |
| `Unsupported program id`       | Program not deployed before running tests                              |

---


