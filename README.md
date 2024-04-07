![localstack](https://github.com/KittenzExe/localstack/assets/67358250/2984d9a8-5318-4c42-afce-27c42314b56d)

A rust program to run multiple projects at once with a single program.

# ⚙️ Usage

Place the executable file along with `localstack.config.json` in a directory of your choice (see setup instructions for more details), you can then run `localstack.exe` or in your terminal and it will launch separate terminal windows for each program specified in `localstack.config.json`.

# 💾 Setup

With both `localstack.exe` and `localstack.config.json` in the same directory, you can configure it with the following format:
```
[
    {
        "directory": "your\\directory\\here",
        "commands": [
            "your commands"
        ]
    }
]
```
(You can also list off multiple commands if needed)

# 🛠️ Development

## 📦 Building and running from source

### Clone the repository

> git clone https://github.com/kittenzexe/localstack.git
> 
> cd localstack

### Build the project

> cargo build --release

### Configure config

> see [Setup](https://github.com/KittenzExe/localstack/new/main?filename=README.md#setup) for localstack.config.json formatting

### Run localstack

> ./target/release/localstack or ./target/release/localstack.exe

# 📖 Licence

Under the [GNU Affero General Public License v3.0](https://github.com/KittenzExe/localstack?tab=AGPL-3.0-1-ov-file)
