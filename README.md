
# Rust CI Starter

A Rust template project that gets you:

* A CLI skeletons for a CLI with subcommands or flat, or a library
* CI and CD (releases) with lint, testing, [and coverage with Codecov](https://blog.rng0.io/how-to-do-code-coverage-in-rust)
* Developer workflow based on [xtask](https://github.com/matklad/cargo-xtask)
* A good base linting configuration, popular libraries (which you can remove), and [insta](https://insta.rs) for tests.



## Getting started

Clone the project or use a tool like [`backpack`](https://github.com/rusty-ferris-club/backpack) to kickstart a new project:

```
$ brew tap rusty-ferris-club/tap && brew install backpack
$ bp new rusty-ferris-club/rust-starter my-project
```

Then take an approach of _removing what you don't need_:

```
`starter_project/` - a CLI with subcommands, based on clap
`starter_project_simpler/` - a CLI with a simple flat structure, based on clap
`starter_project_lib/` - a library-only starter project, with minimal dependencies
```
Once that's done you can update `cargo.toml` and remove crates from the workspace.

Run build to see that you're ready to start:

```
cargo build
```

### Personalising

Find where `starter_project` is and replace it with the name of your project. `$ rg starter_project`, `find . | grep starter_project`.
Find which variables you need to swap. `$ rg __V_`, `$ rg __v_`.
  
Update all snapshots:

```
$ cargo insta test
```
And then

```
$ cargo insta review
````

### README.md

If you like, you can replace this file with a README from the sub-crates:
* [Library README.md](starter_project_lib/README.md)


# Workflow

Workflow for your future project is based on `xtask`.

Install the helper cargo tools with:

```
$ cargo xtask install
```

## Build

```
$ cargo build
```

or, for full build suite with tests:

```
$ cargo xtask ci
```

## Coverage

```
$ cargo xtask coverage [--dev]
```

## Docs

```
$ cargo xtask docs
```

## Release

Set up your [release.yml](.github/workflows/release.yml), replace the project names and repo locations.

Then to release new versions:

* Update [cargo.toml](starter_project/Cargo.toml) version.
* `git tag v[your-new-version]`
* push the new tag

Your Github Action CI workflow should release a new version.

