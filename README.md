![Blazesh](./media/blazesh.jpg)

It is a shell prompt made in Rust. Minimalist, dynamically configurable, a little opinionated

![Blazesh demo](./media/demo.jpg)

## Why

1. As a fun thing
2. I was bored
3. Eye candy
4. It is visually distinct from black and white terminal text, so easier to use
5. Main reason is, though, using [Starship](https://github.com/starship/starship) is for normies. **Real nerds** write their own shell prompts. It's not that hard really. Then I guess I could've kept this repo private, but whatever

And I called it **Blazesh** in reference to the meme, not like it was created to be brazingly fast. If you want something really fast, `unset precmd_functions`

## Installation

You can install it using:

```sh
cargo install --git https://github.com/rdsq/blazesh
```

And then add one of these to your shell config file (just don't forget to add `~/.cargo/bin` to `PATH`):

### Bash `~/.bashrc`

```sh
eval "$(blazesh setup bash)"
```

### Zsh `~/.zshrc`

```sh
eval "$(blazesh setup zsh)"
```

### Bash or Zsh autodetect

```sh
eval "$(blazesh setup detect)"
```

### Fish `~/.config/fish/config.fish`

```fish
source (blazesh setup fish | psub)
```

### Tcsh `~/.tcshrc` / `~/.cshrc`

```csh
eval "`blazesh setup csh`"
```

## Features & Configuration

Here is how you can configure Blazesh with environment variables:

### Git

It shows a git panel, yes it kind of takes time to load, but it is helpful

- `+` represents the uncommitted changes
- `↑` represents unpushed changes
- `↓` represents unpulled changes

You can configure how it handles git by changing the `BLAZESH_GIT_MODE` environment variable. **Possible values:**

- `unoptimized` - check git status every time, even if it is not a git repository
- `optimized` (default) - check git status only if the current directory or one of its parents is a git repository. **Best for functionality**
- `optimized-cwd` - like `optimized`, but checks only the current directory, doesn't check its parents
- `static` - just show `[git]` if the current directory or one of its parents is a git repository. Does not check any status or anything. **Best for performance**
- `static-cwd` - like `static`, but checks only the current directory
- `disabled` - completely disable git integration

*And yes, you can use non 🦅🦅🦅🦅🦅🦅 spelling*

### Colors

(probably the main feature of this project)

You can set the color of the path in the prompt by editing `BLAZESH_ACCENT_COLOR`. You can set it to any number 0-7 and 9 representing the ANSI color codes, and also any RGB value. You can also set it to a sequence of colors, and it will show them as repeating colors. **Examples:**

- `5` - magenta
- `9` - default color (usually white or black)
- `4 3` - 🇺🇦
- `4 4 4 4 3 3 3 3` - same as the previous one, but more readable
- `0 1 2 3 4 5 6 7` - full ANSI rainbow
- `FF0000 FF7F00 FFFF00 00FF00 00FFFF 0000FF 8B00FF` - actual rainbow 🌈

RGB values can be written either as hexadecimal (`FF9900`) or as decimal (`255,153,0`)

**But** it also has the second mode: **gradient**

You can set a gradient between any two or more RGB values by following the `gradient [color1] [color2] ... [color_n]` syntax

For example, set the `BLAZESH_ACCENT_COLOR` environment variable to `gradient 0057B7 FFD700` to see the gradient between the official 🇺🇦 colors

![Really long path showing a gradient from blue to yellow](./media/gradient.jpg)

You can even define gradient looping! This means that the colors will repeat after certain number of characters

```sh
# For RGB
export BLAZESH_ACCENT_COLOR="gradient FF0000 00FF00 0000FF interval=10"
```

And the default color is a looping gradient too! It is `gradient FF9900 FFFF00 interval=10`

And you can define gradient offset. This one is not as visible, but it allows you to add a little dynamic time based glow to it for example

```sh
function _blazesh_time_offset {
    export BLAZESH_ACCENT_COLOR="gradient ff9900 ffff00 interval=10 offset=$(date +%s)"
}
precmd_functions+=(_blazesh_time_offset)
```

And you can also set the accent color to `none` to disable the colors in the path. Not sure why, but you can

### Path

Use `BLAZESH_PATH_DEPTH` to edit how many directories to show before replacing them with `…`. Any number from 0 to 255. **Default: 2**

*(It uses Unicode HORIZONTAL ELLIPSIS `…` instead of three dots `...` to save some screen space. That's the point, after all)*

**Examples with values and how it displays the path**:

- `0` - `…`
- `1` - `…/src`
- `2` *(and higher)* - `~/blazesh/src`

### Exit codes format

You can change how exit codes will be shown in the prompt by editing `BLAZESH_EXIT_CODE_FORMAT`. Possible values:

- `code`: just the code, no message, if you like it serious. Example: `[130]`
- `message`: just show the message if available, good for being compact. Example: `[SIGINT]`
- `both` *(default)*: show both the code and the message. Example: `[130/SIGINT]`

*(does not work in Bash for whatever reason)*

### Path shorthands

By default, it shortens `$HOME` as `~`, but you can define your own custom shorthands with `BLAZESH_PATH_SHORTHANDS` in `path:shorthand` format. Just don't forget to add `$HOME:~` first

For example, `$HOME:~;/root:r~` would show paths as:

- `$HOME/something`: `~/something`
- `/root/something`: `r~/something`

### Non-default shell

This prompt can also show that you're using a shell that is different from the default one set with `chsh`

If you want to disable this, set `BLAZESH_NON_DEFAULT_SHELL` to `disabled`

### ls

This one was made to feel more comfortable using the command line, it shows the contents of your current directory as `(DIRS FILES DOTS)` *(disabled by default)*

You can enable it by setting `BLAZESH_LS=enabled`. You can also configure the dots mode. (dots referring to all items starting with `.`, e.g. `.git`, `.zshrc`)

You can change your preferred dots mode with `BLAZESH_LS` value:

- `dots=ignored`: does not count any *dots*. You can think of it as default `ls`
- `dots=separate` *(default)*: counts dots into a separate category (shown in the example), marked grey. Closest mental model would be `echo * .*`
- `dots=counted`: counts dots just like any other items. Think of it as `ls --all`

## Example configurations

### Insanity

*You have the freedom to make your command prompt look insane if you are*

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

Or random RGB color:

```sh
export BLAZESH_ACCENT_COLOR="$(printf '#%06X\n' $((RANDOM * RANDOM % 16777216)))"
```

Or random gradient:

```sh
# Generate 12 random hex cluster
random_hex=$(xxd -p -l 6 /dev/urandom)

# Split in two
color1="#${random_hex:0:6}"
color2="#${random_hex:6:6}"

export BLAZESH_ACCENT_COLOR="gradient $color1 $color2"
```

### Color of your distro

This will make your prompt the color the developers of your operating system decided (does literally anything use this value?)

```sh
# I don't know how to use sed please don't get mad
distro_color="$(cat /etc/os-release | grep 'ANSI_COLOR=' \
    | sed 's/ANSI_COLOR="38;2;//' | sed 's/"//' | sed 's/;/,/g')"

if [ -n "$distro_color" ]; then
    export BLAZESH_ACCENT_COLOR="$distro_color"
fi
```

And you can do that with so much more, like days of the week, hours, whatever you wish!
