# `urlf` 

A markdown URL formatter that enables you to quickly format GitLab URLs with repository keyword and config the tool for new ones.

## How To Use

To compile: 

`cargo build --release` 

To run put `urlf/target/release/` into your `$PATH` or take `urlf/target/release/urlf` 

then just run `urlf` and it will walk you through a first setup!

### First Run 

On the first run you'll need to do two things: 

1. Set a baseline URL 
2. Create a repository alias (e.g. `core => /dev/core/core`) 

### Normal Use

From there you can start formatting URLs like: 

`urlf \! 42` with will output: 

```
![42](https://gitlab.1password.io//dev/core/core/-/merge_requests/42)
```

You can use many different markers to identify the type of URL you want since `!` can be dangerous to use depending on your `$SHELL`: 

MR: 
-  `mr`
-  `m`
-  `merge request`
-  `merge`
-  `!`

Issues:
-  `i`
-  `issue`
-  `#`

Milestones:
- `milestone`
- `m`
- `$`

If you have multiple repositories already added to your config you can use the repository alias you defined (left hand side of the fat arrow):

```
❯ ./target/release/urlf \! 42 b5
![42](https://gitlab.1password.io//dev/dev/b5/-/merge_requests/42)
```

If none is provided to the tool it will opt for what you have set for the default. 

## How To Config 

Setting a new alias is pretty easy you can do this from the tool itself or by editing the TOML file directly. 

### Through The Tool

run `urlf train x` (it's a bug right now that need the `x`), I'm working on it...slowly :P.

Then you will be asked to create a new alias like: 

`ALIAS => /path/to/repo` 

If you want here you can create a default by: 

`default => /path/to/repo`

This can be an alias that already exists as well.

### Through The Config File 

We store the config file (using `$XDG_HOME`) in `$HOME/.config/urlf/config.toml` edit that in your favorite editor it should look like: 

```
base_url = "gitlab.1password.io"

[aliases]
default = "/dev/core/core"
core = "/dev/core/core"
b5 = "/dev/b5/b5"
```
