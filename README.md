# mojo - a markdown bullet journal manager

Mojo is written in Rust. 

## The Format

### Tasks

Tasks are lines that start with `- [ ]`
They can be nested.

```
- [ ] Task incomplete
- [x] Task complete
- [>] Task migrated
- [<] Task scheduled in Future Log
- [-] A 'deleted' task
```

### Events

Events are represented by the `@` symbol. They may also be
followed up with a descriptor (e.g. `@12pm`, `@lunch`)

```
@ Saw a bird
@ Drank a cappucino
@2:15  Went for a run
```

### Notes

In markdown there is no distinction between `*`, `-`, and `+`. 
For this rason they are all used interchangeably.

```
* This is a ntoe
- Another note
+ Totally a note
```


### Signifiers

TODO

## File Types

### Daily Entries

Filename Format: `$year/$month/$day-$dayOfWeek.md`
e.g.`2019/03/02-saturday.md`

### Monthly Log

Filename Format: `$year/$month/README.md`
e.g.`2019/03/README.md`

### Future Log

TODO

## Commands

```
`mojo` - open todays entry

`mojo -d 2019-02-01` - open the entry from February 1st, 2019

`mojo prev` - open the previous enty
`mojo -1` - open the previous entry
`mojo -5` - open the fifth to last entry

`mojo todo "I have to do something"`` - add task to todays entry

`mojo event "I ate a biscuit"` - add event to todays entry
`mojo now "Drinking coffee"` - add event to todays entry with current time
`mojo event -t "lunch" "Babas cheeseburger"` - add event to todays entry with given time

`mojo note "Billy likes biscuits"` - add note to todays entry

`mojo review` - an interative review of unfinished business
in past entries. Interactive commands
  `x` - mark as complete 
  `f` or `>` - migrate to current day
  `e` - migrate to future log
  `n` or `SPACE`- skip
  `d` - mark as ignored
```

## Developing

**Run Mojo**

```sh
cargo run
```

**Test Mojo**
```sh
cargo test
```

**Format Code with [rustfmt](https://github.com/rust-lang/rustfmt)**
```sh
rustup component add rustfmt
cargo fmt
```

**Lint with [Clippy](https://github.com/rust-lang/rust-clippy)**
```sh
rustup component add clippy
cargo clippy
```

