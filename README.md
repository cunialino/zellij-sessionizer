## Zellij Sessionizer

Trying to solve some stuff i don't like about zellij sessions.

### Problem

I was struggling to create a nice workflow to switch/create zellij sessions, 
I always needed to go through the session-manager default plugin, which is nice, 
but it is a tool I need to open just for this.

### Solution

I created this small (probably buggy) plugin that allows me to have a workflow similar
to [ThePrimeagen Tmux Sessionizer](https://github.com/ThePrimeagen/.dotfiles/blob/master/bin/.local/scripts/tmux-sessionizer).

Basically it allows to start a new session from within a session.

## Configuration

I haven't published a release yet, so for now we need to do the following: 

- Build the wasm and put it in you plugins path:
```bash
git clone https://github.com/cunialino/zellij-sessionizer.git
cd zellij-sessionizer
cargo build --release
cp target/wasm32-wasi/release/sessionizer.wasm <PATH_TO_YOUR_PLUGIN_FOLDER>
```
- Create a plugin alias for this plugin in the zellij kdl config:
```kdl
plugins {
  sessionizer location="file:<PATH_TO_SESSIONIZER_WASM>" {
    cwd "~/"
    // here you can put any additional config
  }
}
```

Available configurations are:
- scrolloff: how much lines to look forward from current selection (like vim)
- find_cmd: the command to list the directories you are interested in.
By default it uses the [fd](https://github.com/sharkdp/fd) command.
Write it in the form "cmd_name;arg1;arg2;..."
- default_dirs: directories to look into, these are appended to the command, format "dir1;dir2;dir3"
- default_layout: layout to use to open sessions, if not specified the default layout your zellij config will be used.

**Note:** the default dirs, if relative paths, will be relatice the the cwd you set for your plugin.

## Usage

As of now there are two options to use this:
1. call `zellij pipe -p sessionizer -n sessionizer-new`, you can call this with or without args:
    - without args: happens the same thing decsribe in point 2 
    - with args: depends on the args described below
        - cwd: If you pass the cwd arg, it will start a session in that cwd with your default layout and named as the dir. 
        - name: Name of the session (optional, defaults to cwd last part)
        - layout: name of the layout you want, defaults to default layout
2. call `zellij action launch-or-focus-plugin sessionizer`:
this will run the [fd](https://github.com/sharkdp/fd) in the `~/` and `~/.config` dirs,
if you type in stuff you fuzzy-filter the directories, and you can navigate them with ctrl+n and ctrl+p (yes i use neovim btw),
hitting enter will create a new session in that folder

## Roadmap

- [ ] CLEAN THE CODE
- [x] accept config for all the params
- [ ] create github release with wasm
- [ ] launch plugin without pipes (is it actually useful?)
- [x] better docs


## Integration with [Yazi](https://github.com/sxyazi/yazi)

This can be used to start/switch sessions from yazi:
- Define a script:
```bash
#! /usr/bin/env bash

LAYOUT=<YOUR_FAV_LAYOUT>

if [ $# -eq 1 ]; then 
  CWD="$1"
  SESSION_NAME="$(basename $CWD)"
else
  CWD="$HOME"
  SESSION_NAME="conf"
fi

if [ ! -z $ZELLIJ ]; then
  zellij pipe -p sessionizer -n sessionizer-new --args cwd="$CWD",name="$SESSION_NAME",layout="$LAYOUT"
else
  zellij attach -c $SESSION_NAME options --default-cwd $CWD --default-layout $LAYOUT 
fi
```

- bind this to a keymap to create a sesison into the hovered directory with the following keymap:

```toml
[manager]
prepend_keymap = [
  { on = [
    "z",
    "s",
  ], run = "shell --block 'zellij_sessionizer.sh $0'", desc = "Start New session in selected dir" },
]
```

## Final notes

When i started writing this I did not google enough, there are plugins like this one:
- [ zellij-sessionizer ](https://github.com/laperlej/zellij-sessionizer): Implements that lists and find directories (better than me), does not allow to create sessions with pipes
- [ zellij-switch ](https://github.com/mostafaqanbaryan/zellij-switch): Implements the switching part with pipes
