# Structure

> _remove when done_

```
bin/
  bumblefoot/    <--- turns into the binary name
    cmd/             <--- a [binary] [command] [options] structure. e.g. git clone foobar.
      mod.rs
      default.rs     <--- the default command
      validate.rs
    main.rs     <--- the main binary, uses library as a, well, library. Register new commands here.
data.rs       <--- types and data structures
runner.rs     <--- one example of library logic
lib.rs        <--- selectively export types to use from the library
```
