# yi

[![Crates.io](https://img.shields.io/crates/v/yi)](https://crates.io/crates/yi)
[![Docs.rs](https://img.shields.io/docsrs/yi)](https://docs.rs/yi)
[![License](https://img.shields.io/crates/l/yi)](https://github.com/CrateProtocol/yi/blob/master/LICENSE)
[![Build Status](https://img.shields.io/github/workflow/status/CrateProtocol/yi/E2E/master)](https://github.com/CrateProtocol/yi/actions/workflows/programs-e2e.yml?query=branch%3Amaster)
[![Contributors](https://img.shields.io/github/contributors/CrateProtocol/yi)](https://github.com/CrateProtocol/yi/graphs/contributors)
[![NPM](https://img.shields.io/npm/v/@crateprotocol/yi)](https://www.npmjs.com/package/@crateprotocol/yi)

<p align="center">
    <img src="https://raw.githubusercontent.com/CrateProtocol/yi/master/images/banner.png" />
</p>

Yi Token by Crate Protocol: the standard for auto-compounding single token staking pools.

## Usage

First, create a Yi Token by invoking the [`yi::create_yi_token`] instruction. Then, anyone may stake
tokens into the pool via [`yi::stake`].

To send auto-compounded rewards to the pool, deposit tokens to the [`YiToken::underlying_tokens`] token account.
This will increase the conversion rate of Yi Tokens to underlying tokens.

To exit the pool, invoke [`yi::unstake`].

### Fees

Yi Tokens may take stake or unstake fees. These fees cannot be changed after the construction of the Yi Token. Fees get distributed
to stakers within the Yi Token pool.

## Packages

- NPM Package: [`@crateprotocol/yi`](https://www.npmjs.com/package/@crateprotocol/yi)
- Crates.io: [`yi`](https://crates.io/crates/yi)

## Address

The Yi program is deployed on `mainnet-beta` and `devnet` at [`YiiTopEnX2vyoWdXuG45ovDFYZars4XZ4w6td6RVTFm`](https://anchor.so/programs/YiiTopEnX2vyoWdXuG45ovDFYZars4XZ4w6td6RVTFm).

## License

Yi Token by Crate Protocol is licensed under the Affero General Public License, version 3.0.
