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

- `ponyrep tmux/tmux`