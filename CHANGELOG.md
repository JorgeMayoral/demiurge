# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2026-03-04

### 📚 Documentation

- Fix git clone url

### ⚙️ Miscellaneous Tasks

- Update release plz config
- Fix release plz workflow
- Fix release plz config
- Fix release plz workflow
- Release v0.3.0 ([#13](https://github.com/JorgeMayoral/demiurge/pull/13))

## [0.3.0](https://github.com/JorgeMayoral/demiurge/compare/v0.0.0...v0.3.0) - 2026-03-04

### Added

- improve logging
- add status subcommand
- implement partial success
- symlink source, not individual files
- validate configs
- remove empty sections from dry run report
- add verbosity flag
- add cargo packages support and improve packages
- update logs format
- add confirmation prompt when applying
- allow apply static configs from stdin
- allow update types file and overwrite initial files
- add user config feature
- add services feature
- add apply from json or yaml feature
- add json schema generation feature
- add schemars dependency
- update metadata
- allow eval to yaml
- allow eval to json
- split applied config files into multiple
- add dotfiles symlink feature
- add command for initializing configuration
- migrate to typescript config with rustyscript
- load config file from given path
- add dry run option
- apply only needed changes
- calculate changes to apply
- install packages
- multiple improvements
- read packages from python config
- allow changing hostname
- initialize project

### Fixed

- return error instead of exit process
- avoid running user related commands when not needed
- remove packages before install to avoid conflicts
- improve apply from options
- improve apply from options
- fix dotfiles changes not created correctly

### Other

- bump version
- *(deps)* bump clap from 4.5.56 to 4.5.60 ([#12](https://github.com/JorgeMayoral/demiurge/pull/12))
- fix dependabot yaml
- add cargo dist
- add dependabot
- add git cliff config
- release v0.2.0
- bump version
- update release-plz workflow
- release v0.1.0
- add release workflows
- add github docs workflow
- add github test workflow
- update documentation
- save applied configs individually
- add license field
- fix release plz image
- fix release plz image
- fix release plz image
- add release workflows
- update documentation
- move system-level side effect methods from config to changes structs
- update documentation
- remove unnecesary comments
- clean tests
- replace unwrap for expect
- fix clippy error
- update tests
- centralize file name constants
- change log msg and remove unnecesary variable
- fix typo
- remove unnecesary clones
- add context to errors
- fix silent errors
- remove unwraps in for each loops
- remove unwraps
- fix docs workflow
- fix docs workflow
- fix docs workflow
- add doc publish workflow
- add changes tests
- add config tests
- add utils tests
- update cli tests
- add tempfile dev dependency
- Add documentation ([#1](https://github.com/JorgeMayoral/demiurge/pull/1))
- update test workflow
- add test worflow
- remove library
- update todos file
- split cli commands into multiple files
- split changes code into multiple files
- update todos file
- update documentation
- add readme
- change some struct names

## [0.2.0](https://github.com/JorgeMayoral/demiurge/compare/v0.0.0...v0.2.0) - 2026-03-03

### Added

- add status subcommand
- implement partial success
- symlink source, not individual files
- validate configs
- remove empty sections from dry run report
- add verbosity flag
- add cargo packages support and improve packages
- update logs format
- add confirmation prompt when applying
- allow apply static configs from stdin
- allow update types file and overwrite initial files
- add user config feature
- add services feature
- add apply from json or yaml feature
- add json schema generation feature
- add schemars dependency
- update metadata
- allow eval to yaml
- allow eval to json
- split applied config files into multiple
- add dotfiles symlink feature
- add command for initializing configuration
- migrate to typescript config with rustyscript
- load config file from given path
- add dry run option
- apply only needed changes
- calculate changes to apply
- install packages
- multiple improvements
- read packages from python config
- allow changing hostname
- initialize project

### Fixed

- return error instead of exit process
- avoid running user related commands when not needed
- remove packages before install to avoid conflicts
- improve apply from options
- improve apply from options
- fix dotfiles changes not created correctly

### Other

- bump version
- update release-plz workflow
- release v0.1.0
- add release workflows
- add github docs workflow
- add github test workflow
- update documentation
- save applied configs individually
- add license field
- fix release plz image
- fix release plz image
- fix release plz image
- add release workflows
- update documentation
- move system-level side effect methods from config to changes structs
- update documentation
- remove unnecesary comments
- clean tests
- replace unwrap for expect
- fix clippy error
- update tests
- centralize file name constants
- change log msg and remove unnecesary variable
- fix typo
- remove unnecesary clones
- add context to errors
- fix silent errors
- remove unwraps in for each loops
- remove unwraps
- fix docs workflow
- fix docs workflow
- fix docs workflow
- add doc publish workflow
- add changes tests
- add config tests
- add utils tests
- update cli tests
- add tempfile dev dependency
- Add documentation ([#1](https://github.com/JorgeMayoral/demiurge/pull/1))
- update test workflow
- add test worflow
- remove library
- update todos file
- split cli commands into multiple files
- split changes code into multiple files
- update todos file
- update documentation
- add readme
- change some struct names

## [0.1.0](https://github.com/JorgeMayoral/demiurge/compare/v0.0.0...v0.1.0) - 2026-03-03

### Added

- implement partial success
- symlink source, not individual files
- validate configs
- remove empty sections from dry run report
- add verbosity flag
- add cargo packages support and improve packages
- update logs format
- add confirmation prompt when applying
- allow apply static configs from stdin
- allow update types file and overwrite initial files
- add user config feature
- add services feature
- add apply from json or yaml feature
- add json schema generation feature
- add schemars dependency
- update metadata
- allow eval to yaml
- allow eval to json
- split applied config files into multiple
- add dotfiles symlink feature
- add command for initializing configuration
- migrate to typescript config with rustyscript
- load config file from given path
- add dry run option
- apply only needed changes
- calculate changes to apply
- install packages
- multiple improvements
- read packages from python config
- allow changing hostname
- initialize project

### Fixed

- return error instead of exit process
- avoid running user related commands when not needed
- remove packages before install to avoid conflicts
- improve apply from options
- improve apply from options
- fix dotfiles changes not created correctly

### Other

- add release workflows
- add github docs workflow
- add github test workflow
- update documentation
- save applied configs individually
- add license field
- fix release plz image
- fix release plz image
- fix release plz image
- add release workflows
- update documentation
- move system-level side effect methods from config to changes structs
- update documentation
- remove unnecesary comments
- clean tests
- replace unwrap for expect
- fix clippy error
- update tests
- centralize file name constants
- change log msg and remove unnecesary variable
- fix typo
- remove unnecesary clones
- add context to errors
- fix silent errors
- remove unwraps in for each loops
- remove unwraps
- fix docs workflow
- fix docs workflow
- fix docs workflow
- add doc publish workflow
- add changes tests
- add config tests
- add utils tests
- update cli tests
- add tempfile dev dependency
- Add documentation ([#1](https://github.com/JorgeMayoral/demiurge/pull/1))
- update test workflow
- add test worflow
- remove library
- update todos file
- split cli commands into multiple files
- split changes code into multiple files
- update todos file
- update documentation
- add readme
- change some struct names
