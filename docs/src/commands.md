# Commands

The `dmrg` binary exposes four subcommands.

## `dmrg init`

Creates the initial configuration files (`index.ts` and `index.d.ts`) in the target directory.

```sh
dmrg init [OPTIONS]
```

| Flag                | Description                                                                    |
|---------------------|--------------------------------------------------------------------------------|
| `-p, --path <PATH>` | Directory where the files will be created. Defaults to the current directory.  |
| `--overwrite`       | Overwrite both files if they already exist.                                    |
| `--update-types`    | Only update `index.d.ts` to the latest version, leaving `index.ts` untouched. |

**Examples:**

```sh
# Initialize in the current directory
dmrg init

# Initialize in a specific directory
dmrg init --path ~/dotfiles

# Overwrite existing files
dmrg init --overwrite

# Update only the type definitions
dmrg init --update-types
```

`--overwrite` and `--update-types` cannot be used together.

---

## `dmrg apply`

Evaluates the configuration, computes the diff against the last applied state, and applies the changes. A confirmation prompt is shown before applying unless `--no-confirm` is passed.

```sh
dmrg apply --name <NAME> [--file <FILE> | --stdin] [OPTIONS]
```

| Flag                  | Description                                                                                    |
|-----------------------|------------------------------------------------------------------------------------------------|
| `-f, --file <FILE>`   | Path to the configuration file. Required unless `--stdin` is used.                            |
| `-n, --name <NAME>`   | Name of the configuration to apply.                                                            |
| `-d, --dry-run`       | Print the list of changes without applying them. Skips the confirmation prompt.                |
| `--no-confirm`        | Skip the confirmation prompt and apply immediately.                                            |
| `--overwrite-symlink` | Allow overwriting already existing dotfile symlinks.                                           |
| `--from-json`         | Parse the configuration as JSON (from file or stdin).                                          |
| `--from-yaml`         | Parse the configuration as YAML (from file or stdin).                                          |
| `--stdin`             | Read the configuration from stdin instead of a file. Requires `--from-json` or `--from-yaml`. |

**Examples:**

```sh
# Apply a TypeScript configuration
dmrg apply --file ~/dotfiles/index.ts --name desktop

# Preview changes without applying
dmrg apply --file ~/dotfiles/index.ts --name desktop --dry-run

# Apply without confirmation prompt
dmrg apply --file ~/dotfiles/index.ts --name desktop --no-confirm

# Apply and overwrite existing symlinks
dmrg apply --file ~/dotfiles/index.ts --name desktop --overwrite-symlink

# Apply from a JSON file
dmrg apply --file config.json --name desktop --from-json

# Apply from stdin (useful for piping from another program)
my-config-generator | dmrg apply --stdin --from-json --name desktop
```

Demiurge compares the desired configuration against the previously applied state and only performs the necessary changes:

| Section    | On add                                          | On remove                              |
|------------|-------------------------------------------------|----------------------------------------|
| `system`   | Sets the hostname                               | —                                      |
| `packages` | Installs via the package manager                | Removes via the package manager        |
| `services` | `systemctl start` + `systemctl enable`          | `systemctl stop` + `systemctl disable` |
| `users`    | `usermod --append --groups`                     | `usermod --remove --groups`            |
| `dotfiles` | Creates symlinks                                | Removes symlinks                       |

After successfully applying, the new state is saved so future runs can compute diffs correctly.

---

## `dmrg eval`

Evaluates the TypeScript configuration file and prints the resulting configuration. Useful for debugging or for exporting to JSON/YAML to be consumed by other tools or applied via stdin.

```sh
dmrg eval --file <FILE> [OPTIONS]
```

| Flag                | Description                                  |
|---------------------|----------------------------------------------|
| `-f, --file <FILE>` | Path to the TypeScript configuration file.   |
| `--json`            | Print the output in JSON format.             |
| `--yaml`            | Print the output in YAML format.             |

**Examples:**

```sh
# Print the parsed configuration as a Rust debug struct
dmrg eval --file ~/dotfiles/index.ts

# Export to JSON
dmrg eval --file ~/dotfiles/index.ts --json > config.json

# Export to YAML
dmrg eval --file ~/dotfiles/index.ts --yaml > config.yaml

# Pipe directly into apply
dmrg eval --file ~/dotfiles/index.ts --json | dmrg apply --stdin --from-json --name desktop
```

---

## `dmrg schema`

Prints the JSON Schema for the Demiurge configuration object. Useful for validating configurations or setting up editor schema support.

```sh
dmrg schema [OPTIONS]
```

| Flag                  | Description                                                                          |
|-----------------------|--------------------------------------------------------------------------------------|
| `-o, --output <PATH>` | Directory where the schema will be saved as `schema.json`. Prints to stdout if omitted. |

**Examples:**

```sh
# Print the schema to stdout
dmrg schema

# Save the schema to a directory
dmrg schema --output ~/dotfiles
```
