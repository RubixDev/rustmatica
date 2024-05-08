# rustmatica

[![Crates.io](https://img.shields.io/crates/v/rustmatica)](https://crates.io/crates/rustmatica)

A rust crate for working with Minecraft litematica files.

## Overview

The two main types of this crate are [`Litematic`] and [`Region`]. See their
documentation for more info.

The
[`examples` directory](https://github.com/RubixDev/rustmatica/tree/main/examples)
contains a few basic examples for how to use this crate.

## Usage with [`mcdata`]

`rustmatica` is tightly coupled with [`mcdata`] and makes use of its traits for
block states, entities, and block entities. By default, schematics will use
[`mcdata`]s "generic" types which store most of their data using
[`fastnbt::Value`]s.

```rust
use rustmatica::Litematic;
use mcdata::util::UVec3;

// type must be declared explicitly for Rust to use the default generics
let schem: Litematic = Litematic::read_file("test_files/axolotl.litematic")?;

// block has type `mcdata::GenericBlockState`
let block = schem.regions[0].get_block(UVec3::new(1, 0, 1));
assert_eq!(block.name, "minecraft:water");
// properties aren't typed
assert_eq!(block.properties["level"], "0");
# Ok::<(), rustmatica::Error>(())
```

But [`mcdata`] also offers more concrete types when enabling certain cargo
features. To use these, add a custom dependency on [`mcdata`] similar to this:

```toml
mcdata = { version = "<version>", features = ["latest", "block-states"] }
```

Then you can use the `mcdata::latest::BlockState` type instead:

```rust
use rustmatica::Litematic;
use mcdata::{util::UVec3, latest::BlockState};
use bounded_integer::BoundedU8;

let schem: Litematic<BlockState> = Litematic::read_file("test_files/axolotl.litematic")?;

// block has type `BlockState`
let block = schem.regions[0].get_block(UVec3::new(1, 0, 1));
assert_eq!(block, &BlockState::Water {
    level: BoundedU8::new(0).unwrap(),
});
# Ok::<(), rustmatica::Error>(())
```
