## Zellij Sessionizer

Trying to solve some stuff i don't like about zellij sessions.

## Usage

Just `cargo build --release` and copy the wasm where you keep your zellij plugins

## Problem statement

I was struggling to create a nice workflow to switch/create zellij sessions, 
I always needed to go through the session-manager default plugin, which is nice, 
but it is a tool I need to open just for this.

## Solution 

I created this small (probably buggy) plugin that allows me to have a workflow similar
to [ThePrimeagen Tmux Sessionizer](https://github.com/ThePrimeagen/.dotfiles/blob/master/bin/.local/scripts/tmux-sessionizer).

Basically it allows to start a new session from within a session.

Now my workflow is as follows:
- Create a plugin alias for this plugin in the zellij kdl config:
```kdl
plugins {
  sessionizer location="file:<PATH_TO_SESSIONIZER_WASM>"
}
```
**Note:** placeholder you define here (sessionizer) **MUST** be used in the -p arg `zellij pipe`.
- Define a script:
```bash
#! /usr/bin/env bash

LAYOUT=simple

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
You can modify this to your likings, I am actually always running in a zellij session (default shell of my terminal), so I always end up in the first branch.

I bind this to a keymap in [yazi](https://github.com/sxyazi/yazi) to create a sesison into the hovered directory with the following keymap:
```toml
[manager]
prepend_keymap = [
  { on = [
    "z",
    "s",
  ], run = "shell 'zellij_sessionizer.sh $0'", desc = "Start New session in selected dir" },
]
```
**Note:** the yazi command doesn't run when outside zellij, I do not know yazi enough to make it open zellij.

Mostly running yazi inside of neovim with [ yazi.nvim ](https://github.com/mikavilpas/yazi.nvim)
