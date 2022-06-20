<!-- remove when done -->
<hr> 

**Using the starter project:**

* find where `bumblefoot` is and replace it with the name of your project.
* This is a _dual library and binary_ project and builds both by default. If this is majorly a library, you want to build binaries selectively see more in [Cargo.toml](Cargo.toml).

Compile a CLI:
```
$ cargo build
```
Compile a library:
```
$ cargo build --no-default-features
```

**Project structure**

_default_ - CLI + lib.

```
bin/
  cmd/
    default.rs    <-- the 'bare' command `$ bumblefoot`
    validate.rs   <-- `$bumblefoot validate`
  bumblefoot.rs       <-- main CLI routing logic, add new commands here declaratively
data.rs      <-- aka 'types.rs'
lib.rs       <-- export some public API
runner.rs    <-- implement some logic here
```

_simplified_ - CLI only. copy `default.rs` into `bumblefoot.rs`, then rename into `main.rs` and put under `src/` and delete the rest.
or `git checkout simplified`. Then, fix imports and namespacing for the commands in `main.rs`, and remove subcommand support.

```
main.rs       <-- main CLI routing logic + default command
data.rs      <-- aka 'types.rs'
lib.rs       <-- export some public API
runner.rs    <-- implement some logic here
```


<hr>
<!-- /remove when done -->



<p align="center">
<br/>
<br/>
<br/>
   <img src="media/cover.png" width="420"/>
<br/>
<br/>
</p>
<p align="center">
<b>:white_check_mark: bumblefoot</b>
<br/>
<b>:cowboy_hat_face: bumblefoot</b>
<br/>
<b>:robot: bumblefoot</b>
<br/>
<hr/>
</p>


<p align="center">
<img src="media/screen.png" width="920"/>
</p>

# :key: Bumblefoot <img src="https://github.com/jondot/bumblefoot/actions/workflows/build.yml/badge.svg"/>



# :rocket: Quick Start

Grab a release from [releases](https://github.com/jondot/bumblefoot/releases), or install via Homebrew:

```
brew tap jondot/tap && brew install bumblefoot
```

## Using bumblefoot




# Thanks

To all [Contributors](https://github.com/jondot/bumblefoot/graphs/contributors) - you make this happen, thanks!


# Copyright

Copyright (c) 2021 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
