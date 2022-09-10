# Grin GUI
This is a very Work-in-Progress implementation of the Grin Core Team's Integrated GUI for both Grin Wallet and Grin Node. 

# Goals

The Grin team has spent countless hours making Grin's infrastructure extremely flexible, with multiple ways of running nodes and wallets and extensive developer APIs and documentation for both.

This project aims to pull all of this work together to create a lightweight, flexible and user-friendly method of using Grin. This includes:

  * Presenting a completely working Single UI for both Grin Node and Grin Wallet.
  * Allowing all wallet transaction operations to be performed in a user-friendly and intuitive manner
  * Create and manage a Grin node in-application if desired, while also retaining the options to communicate with other configured or public nodes.
  * The ability to create, configure and manage multiple wallets and nodes including existing installations

# Status

**NOTHING WORKS AT PRESENT**

* UI Framework [iced-rs](https://github.com/iced-rs/iced) has been selected.
* Overall structure of project is in place based on [ajour](https://github.com/ajour/ajour) as a sample project
* Some refactoring of project structure to better separate GUI elements and events
* Theming, localization, UI scaling is in place
* Windows systray functionality in place

# Current Focus

In contrast to most Grin development, Grin GUI is being developed on Windows, with Windows being the first-class citizen. MacOS and Linux will of course also be supported.

Current work is: 

* Including grin wallet + API 
* First-time 'Out of Box' setup and creation of a grin wallet from the UI
* Wallet + Node configuration options

# Contributing

Yes please! This is an excellent project for anyone wanting to get their feet wet with Grin development. Detailed knowledge of Grin's internals is not required, just a familiarity with Grin's APIs and a willingness to dive into [iced-rs](https://github.com/iced-rs/iced).

See [Grin project contribution](https://github.com/mimblewimble/grin/blob/master/CONTRIBUTING.md) for general guidelines we'll eventually be using, however this project is still far too new for most of this to be relevant.

# Building

## Prerequisites
* rust: Install using rustup: https://rustup.rs
    * Rustc version >= 1.59
    * it is recommended to build using the latest version.
    * If rust is already installed, you can update to the latest version by running `rustup update`

### Windows
* `llvm` must be installed

### Linux
> For Debian-based distributions (Debian, Ubuntu, Mint, etc), all in one line (except Rust):

* ``` sudo apt install build-essential cmake git libgit2-dev clang libncurses5-dev libncursesw5-dev zlib1g-dev pkg-config libssl-dev llvm libfontconfig libfontconfig1-dev```

(instructions not yet complete)

# Acknowledgement

* Thanks to [iced-rs](https://github.com/iced-rs/iced) for a workable native Rust GUI
* Thank you to [ajour](https://github.com/ajour/ajour) for a completely working, non-trivial and tested-in-the-wild iced-rs project to use as a base for development.

# License

GPL 3.0 (for the time being)

Note this differs from the rest of the Grin codebase (which uses Apache 2) due to [ajour](https://github.com/ajour/ajour)'s licensing. Currently attempting to get copyright holder's permission to change this to Apache 2.
