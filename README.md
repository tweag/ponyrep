# Ponyrep

Generate a daily log of a GitHub repository activity

![](./ponyrep.png)

# Building

You need the dependencies `cargo`, `rustc` and `clang` to compile `ponyrep`.

1. `git clone https://github.com/tweag/ponyrep`
2. `cd ponyrep`
3. `cargo build --release`

# Running

The command `ponyrep` takes a single argument which is a combination of `ORG/PROJECT` like `tweag/ponyrep` for our repository.

You need the [GitHub CLI tool](https://cli.github.com/) `gh` in your `$PATH` and configured.  It's used to make API calls without having to deal with tokens.

- `ponyrep tmux/tmux`
