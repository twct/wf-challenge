# wf-challenge

**NOTE: This branch contains changes after the 90 minute timer expired. Just a few minor house-keeping items I wish were in the [main](https://github.com/twct/wf-challenge/tree/main) branch.**

### Getting started with local development

**Install Rust and Cargo**

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add `~/.cargo/bin` to your PATH

**Setup the environment**

```sh
$ cp .env.example .env
```

Edit the `.env` file and supply the expected values

| Key                       | Example Value
| -------------------------- | ------------------------------------------------------------------------- |
| API_ENDPOINT               | https://some_api_host                                                     |

**Run the program**

```sh
$ cargo run
```