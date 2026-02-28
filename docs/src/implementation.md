# Implementation

Demiurge is a Rust CLI that embeds a TypeScript runtime, evaluates user configurations, computes a diff against persisted state, and applies changes via system commands. This page walks through the key technical decisions that make that work.

## TypeScript evaluation via an embedded runtime

The [`rustyscript`](https://crates.io/crates/rustyscript) crate wraps Deno's V8-based JavaScript/TypeScript runtime and embeds it directly in the binary, so no Node.js or Deno installation required.

When `dmrg apply` runs, the config file is loaded as a TypeScript ES module. Demiurge calls the default export (a zero-argument function) and receives a plain JavaScript object back. That object is then deserialized into Rust structs via `serde_json`, bridging the JS world into Rust's type system.

This approach keeps the user-facing surface simple, a plain TypeScript function, while giving the runtime full control over evaluation.

## Type-driven configuration with serde and schemars

All configuration structs (`DemiurgeConfig`, `Packages`, `Services`, `Users`, `Dotfiles`, `System`) derive `serde::Serialize` and `serde::Deserialize`. This gives JSON and YAML import/export for free, which is what powers `dmrg eval --json` and `dmrg apply --from-json`.

The same structs also derive `schemars::JsonSchema`. The `dmrg schema` command outputs a JSON Schema generated directly from these types, so the schema is always in sync with the actual implementation, so no manual maintenance needed.

## Embedded TypeScript type declarations

The `index.d.ts` file that `dmrg init` writes to disk is embedded in the binary at compile time using `include_str!`. This means the type declarations always match the exact version of the tool, with no separate distribution step.

`dmrg init --update-types` rewrites only the type file, leaving the user's `index.ts` untouched. This keeps editor autocompletion and type checking accurate after upgrades.

## Declarative, diff-based apply

Each domain has a `*Changes` struct that takes the new desired state alongside the last applied state and produces a delta (what to add and what to remove). Only the delta is applied. This keeps the operation idempotent: running `dmrg apply` twice in a row with the same config produces no changes on the second run.

After a successful apply, the new state is serialized using [`bitcode`](https://crates.io/crates/bitcode) (a compact binary format) and stored in the XDG data directory, resolved via the [`directories`](https://crates.io/crates/directories) crate. This persisted state is what the next run diffs against.

## External process orchestration

System changes are executed via [`duct`](https://crates.io/crates/duct), which provides a composable API for spawning and chaining processes. Commands are expressed as data structures rather than shell strings, which eliminates shell injection risks and keeps the orchestration layer explicit and testable.

The external tools invoked are: `paru`, `cargo`, `systemctl`, `hostname`, `usermod`, and `groupadd`.

## CLI design

The `dmrg` binary is built with [`clap`](https://crates.io/crates/clap) derive macros. Subcommands, argument groups, and conflicts between flags (such as `--stdin` requiring `--from-json` or `--from-yaml`, or `--overwrite` and `--update-types` being mutually exclusive) are declared as struct and field attributes, keeping the CLI definition close to its implementation.

The `--stdin` flag combined with `--from-json` or `--from-yaml` allows any external program to pipe a configuration directly into `dmrg apply`, making Demiurge composable in larger automation pipelines.
