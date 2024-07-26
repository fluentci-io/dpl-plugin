# Dpl Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/dpl)](https://pkg.fluentci.io/dpl)
[![ci](https://github.com/fluentci-io/dpl-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/dpl-plugin/actions/workflows/ci.yml)

This plugin install and run [dpl](https://github.com/travis-ci/dpl) on your CI/CD pipelines.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm dpl setup
```

## Functions

| Name      | Description                                |
| --------- | ------------------------------------------ |
| setup     | Install dpl                                |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/dpl@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: dpl
    args: |
      setup
- name: Show dpl help
  run: |
    flox activate -- type dpl
    flox activate -- dpl help
```
