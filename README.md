# Blazesh

It is a custom prompt theme for Zsh made in Rust

![Blazesh demo](./media/demo.jpg)

It was created to be minimal, compact, beautiful, smart (compared to a potato), and just a fun experiment

## Installation

Simply clone it somewhere using `git`

```sh
git clone https://github.com/rdsq/blazesh
```

Then add this line to `.zshrc`

```zsh
# Replace `~/blazesh` with your path
source ~/blazesh/prompt.zsh
```

And don't forget to compile it using `cargo build --release`. So the full script to install it:

```sh
git clone https://github.com/rdsq/blazesh ~/blazesh
echo "source ~/blazesh/prompt.zsh" >> ~/.zshrc
cd ~/blazesh
cargo build --release
cd -
```

## Features

The prompt shows a clean path, if it is too long it is cut

It also shows the error exit codes, some of them with special messages

And the git panel, yes it kind of takes time to load, but it is helpful

- `+` represents the uncommitted changes
- `↑` represents unpushed changes
- `↓` represents unpulled chnages

And it also shows the number of background jobs, if it is not `0` like this:

![Blazesh showing the jobs number in brackets as 1 after running `sleep 5` on background](./media/jobs.jpg)

## Configuration

If you want to disable git checking, like for speed, you can set the `BLAZESH_DISABLE_GIT` environment variable. You can set it to anything really. It doesn't check
