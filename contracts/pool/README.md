# Lotter Games

This is a game about Lottery, where anyone can stake any asset to earn profits and, on top of that, have a chance of unexpected rewards, all while enjoying the fun. The most important thing is to have joy and happiness!

## Sea DEX Swap Pool：

The Pool smart contract serves as the primary trading venue for users and mainly contains data such as BaseAsset, QuoteAsset, tick, price, bids, and asks orders. Users can engage in market making and trading, with different fee structures: for instance, market makers have a fee of 0.1%, while traders have a fee of 0.2%.

During trading, matching follows the "price-time priority" principle. There are two types of order placements: market orders and limit orders.

Market orders do not need to specify a price and attempt to match existing orders' prices until funds are exhausted or all target orders are fulfilled. If a market order is not fully fulfilled, the remaining portion is canceled and does not enter the order book.

Limit orders require specifying a price and are executed strictly at the designated price without experiencing slippage. When submitting a limit order, different strategies can be specified:
- Must be entirely fulfilled, otherwise, the trade is canceled.
- Partial fulfillment is allowed, and any unfulfilled portion is added to the order book.
- Partial fulfillment is allowed, and any unfulfilled portion is canceled and not added to the primary order book.
- Cannot be fulfilled entirely, used for market maker orders, and must not result in any trades.

Users can place sell orders (asks) and buy orders (bids), and any unfilled limit orders can be canceled.

## Use Cases



## Architecture Diagram


## Setup

Assuming you have a recent version of Rust and Cargo installed
(via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and cargo-run-script.
Unless you did that before, run this line now:

```sh
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:

## Create a Repo

After generating, you have a initialized local git repo, but no commits, and no remote.
Go to a server (eg. github) and create a new upstream repo (called `YOUR-GIT-URL` below).
Then run the following:

```sh
# this is needed to create a valid Cargo.lock file (see below)
cargo check
git branch -M main
git add .
git commit -m 'Initial Commit'
git remote add origin YOUR-GIT-URL
git push -u origin main
```

## compile to WASM
in the project directory, run the command:
```sh
cargo wasm
```

## optimizer the wasm
```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.13.0
```

## Deploy contract to Archway


## CI Support

We have template configurations for both [GitHub Actions](.github/workflows/Basic.yml)
and [Circle CI](.circleci/config.yml) in the generated project, so you can
get up and running with CI right away.

One note is that the CI runs all `cargo` commands
with `--locked` to ensure it uses the exact same versions as you have locally. This also means
you must have an up-to-date `Cargo.lock` file, which is not auto-generated.
The first time you set up the project (or after adding any dep), you should ensure the
`Cargo.lock` file is updated, so the CI will test properly. This can be done simply by
running `cargo check` or `cargo unit-test`.

