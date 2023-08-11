# playing-cards
### A Rust library for playing cards and poker hand evaluation

[![Crates.io](https://img.shields.io/crates/v/playing-cards.svg)](https://crates.io/crates/playing-cards)
[![Docs.rs](https://img.shields.io/docsrs/playing-cards)](https://docs.rs/playing-cards/)

`playing-cards` is a Rust library for developing card games, ranging from dealing cards from a deck to
hand evaluations.

## DISCLAIMER
This library is still in early development. While I will try to mitgate such a case from occuring,
features may be subject to breaking changes across minor version changes. Be sure to check the docs
for updates on any changes and deprecations.

## Features

### Core

Here is a list of types included within the Core package:

- Card
- CardDeck

### Poker

Here is a list of types included within the Poker feature:

- Evaluators
  - High Evaluator
  - Low Evaluator
  - Omaha Evaluator
  - Omaha Hi-Lo Evaluator (TODO)
  - Dramaha High Evaluator
  - Dramaha 2-7 Evaluator (TODO)
  - Dramahadugi Evaluator (TODO)
  - Dramaha 0 Evaluator (TODO)
  - Dramaha 49 Evaluator (TODO)
  - Shortdeck High Evaluator (TODO)
  - Badugi Evaluator

All the evaluators run on the [CactusKev Perfect Hash algorithm](https://github.com/tangentforks/XPokerEval/tree/master/XPokerEval.CactusKev.PerfectHash).
Originally, the evaluators ran on the 2+2 evaluator, but due to how big the lookup graph is (the
binary encoded file took up 4GB+) and because it increased compile time to over 5 minutes, it was
deemed infeasible despite its super fast evaluations.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for more information.
