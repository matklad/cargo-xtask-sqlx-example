# cargo-xtask-sqlx

Example of using [sqlx-cli](https://crates.io/crates/sqlx-cli) with [cargo-xtask](https://github.com/matklad/cargo-xtask/) pattern.

Run as

```console
$ cargo xtask sqlx database create
```

This will use project-local version of sqlx.

Not that this depends on sqlx-cli-as-a-library. This is likely an unsupported
use-case, not covered by semver. Use at your own risk!
