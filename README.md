# cargo-action

A simple GitHub Action to invoke cargo and subcommands, with cross support. Inputs and usage inspired by action-rs' [cargo action](https://github.com/actions-rs/cargo),
with dtolnay's [rust-toolchain](https://github.com/dtolnay/rust-toolchain) as reference.

Written for use in personal projects (i.e. [bottom](https://github.com/ClementTsang/bottom)), though feel free to use it on your own. Obligatory disclaimer that it may or may not work for your use case though - I'm open to issue reports and PRs however!

## Usage

Note - this action does not install Rust for you, you will want to install a toolchain prior to running this (e.g. [`dtolnay/rust-toolchain`](https://github.com/dtolnay/rust-toolchain)).

### Inputs

**Note: Inputs aren't necessarily checked to be valid by this action!**

| Name            | Description                                                                                                                                                                           |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`       | The `cargo` command to run (e.g. `build`, `test`). Required.                                                                                                                          |
| `toolchain`     | The toolchain to use. Do not include the `+` sign (e.g. `nightly`, `beta`). Defaults to stable.                                                                                       |
| `args`          | What arguments to pass to the cargo/cross command.                                                                                                                                    |
| `use-cross`     | Whether to use cross instead of using cargo. If enabled, cross will automatically be installed if needed.                                                                             |
| `cross-version` | The cross version to use. Only used if `use-cross` is enabled. If this is not set, it will default to the newest stable version of cross. `main` or `git:XYZ` are also valid options. |
| `directory`     | Change to the specified directory prior to execution. Useful if your repo's base folder does not contain your Rust project.                                                           |

## Example

```yaml
name: Test
on: push

jobs:
  test:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v3
    - uses: dtolnay/rust-toolchain@stable
    - uses: ClementTsang/cargo-action@v0.0.3
      with:
        command: test
        args: --lib --bins --benches
        use-cross: true
        cross-version: 0.2.4
```
