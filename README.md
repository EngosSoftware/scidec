# Number converter from scientific notation

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]
[![Contributor Covenant][coc-badge]][coc-url]

[crates-badge]: https://img.shields.io/crates/v/scidec.svg
[crates-url]: https://crates.io/crates/scidec
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: LICENSE-MIT
[mit-license-url]: https://github.com/DecisionToolkit/dsntk-rs/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: LICENSE
[apache-license-url]: https://github.com/DecisionToolkit/dsntk-rs/blob/main/LICENSE
[apache-notice-url]: https://github.com/DecisionToolkit/dsntk-rs/blob/main/NOTICE
[coc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[coc-url]: CODE_OF_CONDUCT.md
[repository-url]: https://github.com/EngosSoftware/scidec

## Overview

This library converts the input text containing a number in scientific notation like:

```text
1234.5678e-2
```

into a number, represented by the following tuple:

```rust
(sign, mantissa-hi-64-bits, mantissa-lo-64-bits, exponent)
```

## Example

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

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to [**scidec**][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
