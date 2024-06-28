# playing-cards
### A Rust library for playing cards and poker hand evaluation

[![Crates.io](https://img.shields.io/crates/v/playing-cards.svg)](https://crates.io/crates/playing-cards)
[![Docs.rs](https://img.shields.io/docsrs/playing-cards)](https://docs.rs/playing-cards/)

## DISCLAIMER
This library is still in early development. While I will try to mitgate such a case from occuring,
features may be subject to breaking changes across minor version changes. Be sure to check the docs
for updates on any changes and deprecations.

## MSRV (Minimum Supported Rust Version)
Requires rustc 1.62+

## Features

### Core

Here is a list of types included within the Core package:

- Value
- Suit
- Card
- CardDeck

### Poker

Here is a list of types included within the Poker feature:

- Evaluators
  - High Evaluator
  - 2-7 Low Evaluator
  - A-5 Low Evaluator (TODO)
  - Omaha High Evaluator
  - Omaha Hi-Lo Evaluator
  - Dramaha High Evaluator
  - Dramaha 2-7 Evaluator (TODO)
  - Dramahadugi Evaluator (TODO)
  - Dramaha 0 Evaluator (TODO)
  - Dramaha 49 Evaluator (TODO)
  - Shortdeck High Evaluator (TODO)
  - Badugi Evaluator

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for more information.
