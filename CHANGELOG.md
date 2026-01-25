# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1](https://github.com/mkaramuk/tohum/compare/v0.3.0...v0.3.1) - 2025-09-14

### Fixed

- wrong sorting of the versions
- add new ts cli template to the store
- old rust channel hash

## [0.3.0](https://github.com/mkaramuk/tohum/compare/v0.2.0...v0.3.0) - 2025-07-16

### Added

- init git repo in the project dir

### Other

- update README

## [0.2.0](https://github.com/mkaramuk/tohum/compare/v0.1.0...v0.2.0) - 2025-07-15

### Added

- Add overwrite flag and fix cross-device link error

## [0.1.0](https://github.com/mkaramuk/tohum/compare/v0.0.2...v0.1.0) - 2025-07-13

### Added

- implement new approach
- change the metadata.json location in the templates
- *(metadata)* change author field type
- add tags for metadata
- *(metadata)* property for template description

### Fixed

- *(store)* refactor store.json structure
- use semver format for template version
- *(flake)* define rust-src path in dev shell
- use version from Cargo.toml

### Other

- add release-plz configuration
- mention the project is under development

## [0.0.2](https://github.com/mkaramuk/tohum/compare/v0.0.1...v0.0.2) - 2025-07-12

### Other

- *(cd)* rename the GitHub action
- add Nix flake

## [0.0.1](https://github.com/mkaramuk/tohum/releases/tag/v0.0.1) - 2025-05-25

### Added

- initial release
- *(template)* template for node.js ts project
- include author in the pre-defined variables
- refactor and better interface
- error message if the target dir is exist
- add target-path argument for project directory customization
- add metadata reader and related structs
- change variable name in all project
- arg parser
- get template command added
- add react-ts template
- cli template for golang
- first example template for Golang CLI
- initial commit

### Fixed

- *(cargo)* remove redundant license file field
- delete metadata.json after the init is done
- *(template)* wrong go version usage
- replace project_name variable
- remove debug logs
- correct dir structure
- remove sub directory inside template
- use variables from the command line
- apply new changes to the tests
- correct error return from main
- update all variables from the files
- missing public fields from structs
- optional name extract fix

### Other

- remove unused tool configs
- use release-plz
- add git-cliff
- fix dagger parameters
- update cd pipeline
- add github action for CD
- .gitkeep file for changelogs dir
- change dagger directory
- dagger CI implemented
- rename the from "maker" to "tohum"
- release
- add more information fields to cargo.toml
- add new template
- fix template identifier
- split command creation to another file
- no need to use absolute path for current dir
- add comments
- update README to include custom path option for project initialization
- add .gitignore patterns and cargo lock file
- Initial commit
