### Juta
**Juta provides robust solution for managing financial operations on blockchain. By leveraging the inherent features of Rust and the CosmWasm platform, we offer an efficient, secure, and scalable framework suitable for a wide variety of decentralized applications in finance, especially in automated management tasks which require interaction among multiple decentralized entities or contracts. The three most important features are:**

**1. Automated Portfolio Management:**
   - Manage assets across different vaults without manual intervention, rebalancing to adhered set thresholds.

**2. Simplified Stake Management:**
   - Users stake CW20 tokens into the contract which then automates the distribution amongst different vaults, streamlining the staking process and potentially optimizing returns.

**3. Decentralized Finance (DeFi) Integration:**
   - Easily integrable into broader DeFi solutions on the Kujira and broader Cosmos ecosystem, contributing functionalities like automated rebalancing and multi-vault management to other financial protocols or products.

### Installation
```sh
# this will produce a wasm build in ./target/wasm32-unknown-unknown/release/YOUR_NAME_HERE.wasm
cargo wasm

# this runs unit tests with helpful backtraces
RUST_BACKTRACE=1 cargo unit-test

# auto-generate json schema
cargo schema

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer-arm64:0.15.0

pond init

pond start

pond deploy artifacts/juta.wasm --label juta --admin kujira1cyyzpxplxdzkeea7kwsydadg87357qnaww84dg --from kujira1cyyzpxplxdzkeea7kwsydadg87357qnaww84dg

pond tx wasm instantiate 11 '{"ghost_token": "kujira1cyyzpxplxdzkeea7kwsydadg87357qnaww84dg", "ghost_vaults": ["kujira18s5lynnmx37hq4wlrw9gdn68sg2uxp5r39mjh5", "kujira1qwexv7c6sm95lwhzn9027vyu2ccneaqa5xl0d9"], "threshold": "1", "count":1}' --label test --admin kujira14hcxlnwlqtq75ttaxf674vk6mafspg8xhmzm0f --from kujira14hcxlnwlqtq75ttaxf674vk6mafspg8xhmzm0f

pond q tx CEB117A1CDB08CE6BD117525A601227FE4C6E6055430F917FEF3B4E3AA87BD68
```

# CosmWasm Starter Pack

This is a template to build smart contracts in Rust to run inside a
[Cosmos SDK](https://github.com/cosmos/cosmos-sdk) module on all chains that enable it.
To understand the framework better, please read the overview in the
[cosmwasm repo](https://github.com/CosmWasm/cosmwasm/blob/master/README.md),
and dig into the [cosmwasm docs](https://www.cosmwasm.com).
This assumes you understand the theory and just want to get coding.

## Creating a new repo from template

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

**Latest**

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME
```

For cloning minimal code repo:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME -d minimal=true
```

You will now have a new folder called `PROJECT_NAME` (I hope you changed that to something else)
containing a simple working contract and build system that you can customize.

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

## Using your project

Once you have your custom repo, you should check out [Developing](./Developing.md) to explain
more on how to run tests and develop code. Or go through the
[online tutorial](https://docs.cosmwasm.com/) to get a better feel
of how to develop.

[Publishing](./Publishing.md) contains useful information on how to publish your contract
to the world, once you are ready to deploy it on a running blockchain. And
[Importing](./Importing.md) contains information about pulling in other contracts or crates
that have been published.

Please replace this README file with information about your specific project. You can keep
the `Developing.md` and `Publishing.md` files as useful references, but please set some
proper description in the README.
