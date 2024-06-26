## Installation

### Cargo

You will need to install rust and cargo to run this cli. You can do so by following the instructions [here](https://www.rust-lang.org/tools/install).
```bash
cargo install eas-env
```

### CURL
```bash
curl -sSL https://raw.githubusercontent.com/corasan/eas-env/main/install.sh | bash
```

### Releases
Download the latest release from the [releases](https://github.com/corasan/eas-env/releases) page and add it to your path.

## Usage

By default, the program will use the `default` profile in your `eas.json`. You can specify a different environment by passing the profile as an argument. Note that this needs to be run in the root of your project and you will need to add your environment variables to the EAS profile.

```bash
eas-env
# creates .env.local file using default profile

eas-env -p production
# creates .env.local file using production profile

```

You can also specify the path to the app's directory where the `eas.json` file is located, useful if the project is in a monorepo. The default path is the current directory where the command is run.

```bash
eas-env --app-dir apps/mobile
# creates .env.local file using default profile inside apps/mobile directory
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
