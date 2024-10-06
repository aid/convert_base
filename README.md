# convert_base

This crate has been created as part of a learning exerise - in Rust itself
and also creating crates and documenting them using rustdoc.

The functionailty provided here is available in Rust's standard library -
as such I would not recommend this is used for anything other than playing
around!  Accordingly, this crate hasn't been pushed to `crates.io` and is
not supported code.

The crate provides the following functions:

* **to_number_from_base** - Converts a string of given base to an integer.
* **to_base_from_number** - Converts a number to a string of given base.
* **convert_to_base** - Converts a string from a given base directly to a string of a
  nominated base.

The functions provided by this crate support bases in the range of 2 (binary)
through 16 (hexadecimal).
