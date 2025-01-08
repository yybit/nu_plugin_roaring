[![crates.io](https://img.shields.io/crates/v/nu_plugin_roaring.svg)](https://crates.io/crates/nu_plugin_roaring)
[![docs.rs](https://docs.rs/nu_plugin_roaring/badge.svg)](https://docs.rs/nu_plugin_roaring)

# nu_plugin_roaring

`nu_plugin_roaring` is a plugin for the Nushell that provides efficient bitmap operations using the [Roaring Bitmap](https://roaringbitmap.org/) data structure.

## Installation

```sh
cargo install nu_plugin_roaring
plugin add ~/.cargo/bin/nu_plugin_roaring
plugin use roaring
```

### Usage

The plugin supports the following methods: `new`, `len`, `list`, `contains`, and `serialize`. Here are some examples:

- Create a new bitmap, serialize it, and save it to a file:

    ```sh
    [1..5 11..<15 21] | roaring new | roaring ser | save test.rr
    ```

- Import a serialized bitmap and get its length:

    ```sh
    open test.rr | roaring len
    ```

- Import a serialized bitmap and check if it contains certain values:

    ```sh
    open test.rr | roaring contains 1..3
    ```


- Import a serialized bitmap and list certain values:

    ```sh
    # List elements in the range from 1 to 15
    open test.rr | roaring list 1..15
    # List all elements
    open test.rr | roaring list
    ```

