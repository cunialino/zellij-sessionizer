## Zellij Sessionizer

Trying to solve some stuff i don't like about zellij sessions.

## Usage

Just `cargo build --release` and copy the wasm where you keep your zellij plugins

As of now there are two options to use this:

1. call `zellij pipe` with a cwd args pointing to the dir you want to session into: useful when scripting for other tools (like yazi)
2. call `zellij pipe` without args: this will run `fd` command on default dirs (home + .config as of now) and list the sessions.

## Roadmap

- [ ] CLEAN THE CODE
- [ ] accept config for all the params
- [ ] create github release with wasm
- [ ] launch plugin without pipes (is it actually useful?)
- [ ] better docs

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
You can modify this to your likings, I am actually always running in a zellij session (default shell of my terminal), so I always end up in the first branch.

I bind this to a keymap in [yazi](https://github.com/sxyazi/yazi) to create a sesison into the hovered directory with the following keymap:
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

When i started writing this I did not google enough, there are plugins like this one (written better):
- [ zellij-sessionizer ](https://github.com/laperlej/zellij-sessionizer)
- [ zj-smart-sessions ](https://github.com/dj95/zj-smart-sessions/tree/main)
- [ zellij-switch ](https://github.com/mostafaqanbaryan/zellij-switch)
