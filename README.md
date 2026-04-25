# SkillLock

Freelance escrow and portable reputation powered by Stellar.

## Problem

Freelancers risk non-payment.  
Clients risk hiring unknown contractors.

## Solution

Escrow payments + automated milestone releases + portable reputation credentials.

## Timeline

| Week | Milestone |
|------|-----------|
| 1 | Escrow contract |
| 2 | Frontend MVP |
| 3 | Wallet integration |
| 4 | Hackathon demo polish |

## Stellar Features

- XLM / USDC Transfers
- Soroban Contracts
- Custom Tokens
- Trustlines

## Vision and Purpose

Trustless infrastructure for the global gig economy.

## Prerequisites

- Rust Toolchain
- Soroban CLI

## Build

```bash
soroban contract build
```

## Test

```bash
cargo test
```

## Deploy

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/skilllock.wasm
```

## Sample Invoke

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  -- create_escrow \
  --job_id 1 \
  --amount 50

soroban contract invoke \
  --id CONTRACT_ID \
  -- release_payment \
  --job_id 1
```

## License

MIT

✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/83bbd0721a929fb2e1c77a3ad75d72da4e1f62792addc5c7d15dfd89840d0e31
🔗 https://lab.stellar.org/r/testnet/contract/CCD2F64NCDGY5FJFVUP674Q5ZUBEFDRX3IVOE2QUF6VUCEXDMG6RA6BH
✅ Deployed!
CCD2F64NCDGY5FJFVUP674Q5ZUBEFDRX3IVOE2QUF6VUCEXDMG6RA6BH