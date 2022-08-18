> remove when done
<hr> 

## Using the starter project

Use [`backpack`](https://github.com/rusty-ferris-club/backpack) to kickstart a new project:

```
$ bp new rusty-ferris-club/rust-starter my-project
```

Then,

* Remove the crates you don't need. You can pick from:
  * **`starter_project/`** - a full CLI with subcommands
  * **`starter_project_simpler/`** - a CLI with a simple flat structure
  * **`starter_project_lib/`** - a library only starter project, minimal dependencies
* Update `cargo.toml` and remove crates from workspace

Run build to see that you're ready to start:

```
cargo build
```

* find where `starter_project` is and replace it with the name of your project. `$ rg starter_project`, `find . | grep starter_project`.
  
Update all snapshots:

```
$ cargo insta test
```
And then

```
$ cargo insta review
````


## Workflow
### Build
```
$ cargo build
```
or, for full build suite with tests:

```
$ cargo xtask ci
```
### Test
```
$ cargo build
```

### Coverage
```
$ cargo xtask coverage [--dev]
```

### Release

Set up your [release.yml](.github/workflows/release.yml), replace the project names and repo locations.

Then to release new versions:

* Update [cargo.toml](starter_project/Cargo.toml) version.
* `git tag v[your-new-version]`
* push the new tag

Your Github Action CI workflow should release a new version.



<hr>

> /remove when done



<p align="center">
<br/>
<br/>
<br/>
   <img src="media/cover.png" width="420"/>
<br/>
<br/>
</p>
<p align="center">
<b>:white_check_mark: starter_project</b>
<br/>
<b>:cowboy_hat_face: starter_project</b>
<br/>
<b>:robot: starter_project</b>
<br/>
<hr/>
</p>


<p align="center">
<img src="media/screen.png" width="920"/>
</p>

# :key: starter_project <img src="https://github.com/jondot/starter_project/actions/workflows/build.yml/badge.svg"/>



# :rocket: Quick Start

Grab a release from [releases](https://github.com/jondot/starter_project/releases), or install via Homebrew:

```
brew tap jondot/tap && brew install starter_project
```

## Using starter_project




# Thanks

To all [Contributors](https://github.com/jondot/starter_project/graphs/contributors) - you make this happen, thanks!


# Copyright

Copyright (c) 2021 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
