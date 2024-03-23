## Installation

You will need to [install rust](https://www.rust-lang.org/tools/install) and cargo to run this project. You can do so by following the instructions .

```bash
cargo install eas-env
```

## Usage

By default the program will run in the `default` profile in your `eas.json`. You can specify a different environment by passing the profile as an argument. Note that this needs to be run in the root of your project.

```bash
eas-env
// creates .env.local file with the default profile

eas-env production
```
