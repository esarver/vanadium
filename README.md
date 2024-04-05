# vanadium
A pure-rust vim-like editor library. The goal is to provide an easy to use
plugin system similar to that of the Bevy game engine that makes it possible
to build your entire config in Rust, no scripting languages.

## Requirements

- 100% portable to Linux, Windows, and macOS
- Simple plugin system
- Batteries included, but replaceable
    - LSP, DAP, TreeSitter integrations by default, but replaceable or
      removeable, if desired.
    - LSP, DAP, and TreeSitter all have many default languages, configurable by
      feature flags

## Ideas

- Plugable interface
  - Status line
  - LSP Client
  - DAP Client
  - Keymap Handler
  - View Manager
  - Editor Modes
  - Editor Types

### Design

- Use the `bevy_ecs` crate
- Use Ratatui for the terminal side, potentially going a level lower
  CrossTerm if necessary
- Use keymap tree
    - Each node of the tree can open an informational dialog (a la Helix)
    - Upon reaching a leaf of the tree, an ECS event is fired which will trigger
      a system

