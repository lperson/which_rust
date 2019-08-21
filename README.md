# which_rust
`which_rust` is a tiny utility that tells you which version of Rust is active in the current directory, if any.

[@1000000000](https://github.com/1000000000) and I created the first iteration of `which_rust` in a pair-programming session at [The Recurse Center](http://www.recurse.com).

We designed it to be part of `PS1` in shell profiles, so you can tell, just by looking at the prompt in your shell, which version of Rust you're using, inspired by `pyenv` for python.

Here's an example of a prompt that includes `which_rust`.

## Installation
0. These instructions assume you have Rust [installed](https://www.rust-lang.org/tools/install) on your machine.
1. Clone or download this repository.
2. In the root directory of the repository, execute the following commands:
    - `cargo build`
    - `cargo install --path .`

## Usage

Include `\$(which_rust)` in `PS1` in your shell configuration script (`.bash_profile`, `.bashrc`, `.zshrc`, etc.) if you're using Bash or Zsh.  There is thorough information about customing your Bash prompt [here](https://wiki.archlinux.org/index.php/Bash/Prompt_customization).  Search the web for instructions if you're using a different shell.

## Details

`which_rust` will return nothing if the current directory doesn't seem to be part of a Rust project.  We assume that a directory that contains `Cargo.toml`, or is a child of a directory that contains `Cargo.toml`, is part of a Rust project.  (Note that if for some reason there is a `Cargo.toml` in your home directory, or the root directory of a filesystem, we don't consider every child directory to be part of a Rust project.) 

## Pull requests
Welcome and encouraged.


