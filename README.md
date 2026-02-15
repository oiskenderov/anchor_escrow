````markdown
# Anchor Escrow Program

**Anchor Escrow Program** is a Solana smart contract (program) built using the **Anchor framework**.  
It implements a simple on-chain escrow logic where two parties can securely exchange SPL tokens without needing trust in a third party.

*This project is inspired by/structured similarly to existing Anchor escrow examples (e.g., ironaddicteddog’s anchor-escrow).* :contentReference[oaicite:1]{index=1}

---

## 🚀 Table of Contents

- 🔍 What is this?
- 🧠 Features
- 🧩 Architecture Overview
- ⚙️ Prerequisites
- 📌 Installation
- 🧪 Testing
- 🚢 Deployment
- 📝 Usage Guide
- 📦 Directory Structure
- 🤝 Contributing
- 🛡 License
- 📞 Contact / Support

---

## 🔍 What is this?

This repository contains:

- A Solana escrow program written in Rust using **Anchor**
- TypeScript tests to validate the escrow instructions
- Configs and tooling for running locally and on Devnet

An escrow program lets a maker deposit tokens into an on-chain vault and allows a taker to complete the swap only when both sides meet the agreed upon conditions.

---

## 🧠 Features

✔ **Secure token swap** between parties  
✔ **Program Derived Address (PDA)** controlled vault — no private key control  
✔ Automated state management (close accounts after use)  
✔ Typescript test suite for automated integration testing

---

## 🧩 Architecture Overview

A typical Anchor Escrow flow has three instructions:

1. `initialize` — maker transfers token A into an escrow vault   
2. `cancel` — maker gets token A back if escrow is not taken  
3. `exchange` — taker transfers token B, maker receives token B, taker receives token A

The program ensures a trustless swap where tokens are only released when both conditions are satisfied.

---

## ⚙️ Prerequisites

Before setup, install the following:

- **Solana CLI** >= latest (install via Solana docs)
- **Rust & Cargo**
- **Anchor CLI**  
  ```bash
  cargo install --locked anchor-cli
````

* **Node.js + Yarn**
* A Solana wallet (e.g., local keypair)

---

## 📌 Installation

1. Clone the repository:

```bash
git clone https://github.com/oiskenderov/anchor_escrow.git
cd anchor_escrow
```

2. Install dependencies:

```bash
yarn install
```

3. Confirm Anchor setup

```bash
anchor --version
solana --version
```

4. Build the program

```bash
anchor build
```

---

## 🧪 Testing

To run integration tests (TypeScript based):

```bash
anchor test
```

or, if using Mocha directly:

```bash
yarn test
```

> Tests automatically start a local validator and simulate contract calls.

---

## 🚢 Deployment

Deploy to a Solana cluster:

1. Set network

```bash
solana config set --url devnet
```

2. Airdrop SOL

```bash
solana airdrop 2
```

3. Deploy the program

```bash
anchor deploy
```

4. Copy the deployed program ID into your Anchor config

---

## 📝 Usage Guide

After deployment:

### Initialize Escrow

* Maker initializes escrow with desired token amounts

### Exchange

* Taker deposits the matching token into escrow
* Program swaps tokens and closes state

### Cancel Escrow

* Maker can cancel and refund if not taken

*(Refer to tests for exact instruction calls and accounts.)*

---

## 📦 Directory Structure

```
anchor_escrow/
├─ programs/
│  └─ escrow_testing/
├─ tests/
│  └─ *.ts
├─ Anchor.toml
├─ Cargo.toml
├─ tsconfig.json
├─ yarn.lock
└─ ...
```

---

## 🤝 Contributing

Contributions welcome! To contribute:

1. Fork the repo
2. Create a feature branch
3. Write tests
4. Open a Pull Request

Please follow standard GitHub contribution guidelines.

---

## 🛡 License

This project is licensed under the **MIT License** — see the [LICENSE](./LICENSE) file for details.

---

## 📞 Contact / Support

Created by **oiskenderov**.
If you have questions or need help understanding the Anchor program flow, feel free to open an issue or reach out!

---

```

---

If you’d like, I can also generate:

✅ A **Contributing.md**  
✅ A **Security.md**  
✅ A **Code of Conduct**

Would you like those added?
::contentReference[oaicite:2]{index=2}
```

