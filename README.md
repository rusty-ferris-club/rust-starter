<!-- remove when done -->
<hr> 

**Using the starter project:**

* find where `bumblefoot` is and replace it with the name of your project. `$ rg bumblefoot`
* This is a _dual library and binary_ project and builds both by default.

Compile a CLI:
```
$ cargo build
```
Compile a library:
```
$ cargo build --no-default-features
```

**Project structure**


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

**Simpler structure**
You can convert the project to be simpler and CLI only on account of power and flexibility. 

* Copy the contents of `default.rs` into `bumblefoot.rs` and rename into `main.rs`.  
* Drop `main.rs` under `src/` and delete `bin/`.
* Remove the `[[features]]` and `[[bin]]` sections from `Cargo.toml`
* Fix `use` issues and stale code errors in `main.rs`.
  

This should be the result:

```
bumblefoot/
  main.rs       <-- main CLI routing logic + default command
  data.rs      <-- aka 'types.rs'
  lib.rs       <-- export some public API
  runner.rs    <-- implement some logic here
```

**xtask**
You have [xtask](https://github.com/matklad/cargo-xtask) preconfigured. It's a best-practice, boilerplate code that allows you to use `cargo xtask <your task>`. A kind of rust-native make.

You can use it to codify any of your tasks for CI or development.

In it you have two tasks preconfigured:

* `dual` - convert this crate to a dual build (library and CLI)
* `simple` - convert this crate to a simple layout

These will work only when you're starting out because they apply fresh templated code. Once you start building, they'll probably not be useful anymore.


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
