# Monea
Monea's core user-facing product

## Development

When running the CLI, make sure to do so from the root of the repository, and use the following command:

```bash
cargo run --manifest-path interfaces/cli/Cargo.toml --
```

For example, to run the `help` command:

```bash
cargo run --manifest-path interfaces/cli/Cargo.toml -- --help
```
## Disclosures

Since we are still early in development, we are relying on various pre-existing tools. One of which is [Kurtosis](https://github.com/kurtosis-tech/kurtosis), used under-the-hood strictly for container orchestration. Shoutout to them. :)
