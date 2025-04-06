# Blazesh

It is a custom shell prompt theme made in Rust

![Blazesh demo](./media/demo.jpg)

It was created to be minimal, compact, beautiful, smart (compared to a potato), and just a fun experiment

## Installation

Simply clone it somewhere using `git`

```sh
git clone https://github.com/rdsq/blazesh
```

Then add this line to `.zshrc` or `.bashrc`

```zsh
# Replace `~/blazesh` with your path
source ~/blazesh/blazesh.sh
```

And don't forget to compile it using `cargo build --release`. So the full script to install it:

```sh
git clone https://github.com/rdsq/blazesh ~/blazesh
echo "source ~/blazesh/blazesh.sh" >> ~/.zshrc
echo "source ~/blazesh/blazesh.sh" >> ~/.bashrc
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
- `↓` represents unpulled changes

And it also shows the number of background jobs if it is not `0` like this:

![Blazesh showing the jobs number in brackets as 1 after running `sleep 5` on background](./media/jobs.jpg)

## Configuration

Here is how you can configure Blazesh with environment variables:

### Git mode

You can configure how it handles git by changing the `BLAZESH_GIT_MODE` environment variable. **Possible values:**

- `unoptimized` - check git status every time, even if it is not a git repository
- `optimized` (default) - check git status only if the current directory or one of its parents is a git repository. **Best for functionality**
- `optimized-cwd` - like `optimized`, but checks only the current directory, doesn't check its parents
- `static` - just show `[git]` if the current directory or one of its parents is a git repository. Does not check any status or anything. **Best for performance balance**
- `static-cwd` - like `static`, but checks only the current directory
- `disabled` - completely disable git integration

*And yes, you can use non 🦅🦅🦅🦅🦅🦅 spelling*

### Colors

You can set the color of the path in the prompt by editing `BLAZESH_ACCENT_COLOR`. You can set it to any number 0-7 and 9 representing the ANSI color codes, and also any RGB HEX value. The default is `FF9900` (orange). You can also set it to a sequence of colors, and it will show them as repeating colors. **Examples:**
    - `5` - magenta
    - `9` - default color (usually white or black)
    - `4 3` - 🇺🇦
    - `4 4 4 4 3 3 3 3` - same as the previous one, but more readable
    - `0 1 2 3 4 5 6 7` - full ANSI rainbow
    - `FF0000 FF7F00 FFFF00 00FF00 00FFFF 0000FF 8B00FF` - actual rainbow 🌈

**But** it also has the second mode: **gradient**

You can set a gradient between any two RGB values by following the `gradient [color1] [color2]` syntax

For example, set the `BLAZESH_ACCENT_COLOR` environment variable to `gradient 0057B7 FFD700` to see the gradient between the official 🇺🇦 colors

![Really long path showing a gradient from blue to yellow](./media/gradient.jpg)

### Path

Use `BLAZESH_PATH_DEPTH` to edit how many directories to show before replasimg them with `...`. Any number from 0 to 255. **Default: 2**

**Examples with values and how it displays the path**:

- `0` - `...`
- `1` - `.../src`
- `2` *(and higher)* - `~/blazesh/src`

## Example configurations

### Insanity

*You have the fredom to make your command prompt look insane if you are*

![Command prompt showing path of ~/mydir/another/one/so/many/dirs/im/insane/i/mean/imnot/im/just/showingwhat/aninsaneperson/woulddo in full and repeating ANSI rainbow](./media/insanity.jpg)

```sh
export BLAZESH_ACCENT_COLOR='0 1 2 3 4 5 6 7'
export BLAZESH_PATH_DEPTH=255
```

### Random color every session

You can put this to your `.bashrc`/`.zshrc` or wherever you store your configs and get a new accent color every time you open the shell

```sh
ansi_colors=(0 1 2 3 4 5 6 7)
export BLAZESH_ACCENT_COLOR=$(printf "%s\n" "${ansi_colors[@]}" | shuf -n 1)
```

And you can do that with so much more, like days of the week, hours, whatever you wish!

## Problems

The most obvious one: **synchronous git**. Unfortunately I am not smart enough to figure out how to do that asynchronously, so... Well it's not *that* bad. Even on my 🥔 it runs fast enough to be usable daily

Not so critical one, but also worth mentioning: **exit codes** don't show up in **Bash**. As far as I can tell, this issue cannot be fixed, it's just how Bash works
