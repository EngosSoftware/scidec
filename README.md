# Number converter from scientific notation

The library converts the input text containing a number in scientific notation, e.g:

```
1234.5678e-2
```

to number, represented by the following tuple:

```
(sign, mantissa-hi-64-bits, mantissa-lo-64-bits, exponent)
```

Example:

```rust
use scidec::{Number, number_from_string};

let result = number_from_string("1234.5678e-2");
match result {
  Number::Fin(false, 0, 12345678, -6) => {}
  _ => panic!()
}
```

## License

Licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](https://github.com/wisbery/scidec/blob/main/LICENSE-MIT)), or
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](https://github.com/wisbery/scidec/blob/main/LICENSE-APACHE))

at your option.

## Contribution

All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
