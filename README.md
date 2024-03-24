## Installation

You will need to install rust and cargo to run this cli. You can do so by following the instructions [here](https://www.rust-lang.org/tools/install).

```bash
cargo install eas-env
```

## Usage

By default, the program will use the `default` profile in your `eas.json`. You can specify a different environment by passing the profile as an argument. Note that this needs to be run in the root of your project and you will need to add your environment variables to the EAS profile.

```bash
eas-env
# creates .env.local file using default profile

eas-env -p production
# creates .env.local file using production profile

```

To make things easier you can add it to your `package.json`:

```json
{
  "scripts": {
    "env:dev": "eas-env -p development",
    "env:prod": "eas-env -p production",
    "env:preview": "eas-env -p preview",
  }
}
```

Then you can run `npm run env` to create the `.env.local` file with the default profile.
