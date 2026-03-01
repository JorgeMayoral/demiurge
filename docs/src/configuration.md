# Configuration

Demiurge configurations are written in TypeScript. A configuration file exports a function that returns a `Demiurge` object — a map of named configurations, each describing a desired system state.

## Getting started

Run `dmrg init` in any directory to generate the starter files:

```sh
dmrg init
```

This creates two files:

- **`index.ts`** — your configuration, ready to edit
- **`index.d.ts`** — type declarations for editor support and type checking

## Configuration format

A configuration file must have a default export: a function with no arguments that returns a `Demiurge` object.

```typescript
const config: DemiurgeConfig = {
  system: {
    hostname: "my-machine",
  },
  packages: {
    paru: ["git", "curl", "neovim"],
    cargo: ["cargo-watch"],
  },
  services: ["docker", "bluetooth"],
  users: [
    { name: "alice", groups: ["docker", "video"] },
  ],
  dotfiles: [
    { source: "~/dotfiles/nvim", target: "~/.config/nvim" },
  ],
};

export default (): Demiurge => ({
  "my-config": config,
});
```

## Named configurations

The top-level `Demiurge` object is a key-value map where each key is the name of a configuration. This lets you keep multiple configurations in a single file — for example, one per machine:

```typescript
export default (): Demiurge => ({
  "desktop": desktopConfig,
  "laptop": laptopConfig,
});
```

When applying, you select which configuration to use with the `--name` flag:

```sh
dmrg apply --file index.ts --name desktop
```

## Configuration sections

### `system`

Controls system-level settings.

| Field      | Type     | Description              |
|------------|----------|--------------------------|
| `hostname` | `string` | The desired system hostname |

```typescript
system: {
  hostname: "my-machine",
}
```

### `packages`

Declares packages to be installed, grouped by package manager. The key is the package manager name and the value is the list of package names.

```typescript
packages: {
  paru: ["git", "curl", "neovim"],
  cargo: ["cargo-watch", "cargo-expand"],
}
```

Supported package managers:

| Key     | Install command          | Remove command    |
|---------|--------------------------|-------------------|
| `paru`  | `paru -S <pkgs>`         | `paru -Rs <pkgs>` |
| `cargo` | `cargo install --locked` | `cargo uninstall` |

Packages present in the previously applied config but absent from the new one will be removed.

### `services`

Declares systemd services to keep enabled. Each entry is a service name string.

```typescript
services: ["docker", "bluetooth", "sshd"],
```

Services are started and enabled with `systemctl`. Services present in the previously applied config but removed from the new one will be stopped and disabled.

### `users`

Manages user group membership.

| Field    | Type       | Description                          |
|----------|------------|--------------------------------------|
| `name`   | `string`   | The username                         |
| `groups` | `string[]` | Groups the user should belong to     |

```typescript
users: [
  { name: "alice", groups: ["docker", "video", "input"] },
],
```

Groups that don't exist yet will be created automatically. Groups present in the previously applied config but removed from the new one will be removed from the user.

### `dotfiles`

Manages dotfiles as symlinks. Each entry maps a source directory or file to a target location.

| Field    | Type     | Description                                |
|----------|----------|--------------------------------------------|
| `source` | `string` | Path to the source file or directory       |
| `target` | `string` | Path where the symlink(s) will be created  |

```typescript
dotfiles: [
  { source: "~/dotfiles/nvim", target: "~/.config/nvim" },
  { source: "~/dotfiles/zsh/.zshrc", target: "~/.zshrc" },
]
```

When `source` is a directory, Demiurge recursively walks it and creates a symlink for each file at the corresponding path under `target`. Tilde (`~`) expansion is supported in both fields.

## Validation

Before applying any changes, `dmrg apply` validates the configuration and reports all violations together. Nothing is applied until the configuration passes validation.

| Section    | Rule |
|------------|------|
| `system`   | `hostname` must not contain whitespace or `/`. An empty string is allowed and means "no change". |
| `packages` | The package manager name and each package name must be non-empty. |
| `services` | Each service name must be non-empty. |
| `users`    | The user name and every group name must be non-empty. |
| `dotfiles` | Both `source` and `target` must be non-empty. |

## Alternative formats

If you prefer not to use TypeScript at runtime, you can export your configuration as JSON or YAML using `dmrg eval`, then apply it from a file or via stdin with `--from-json` or `--from-yaml`. See the [Commands](./commands.md) page for details.
