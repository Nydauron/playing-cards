# playing-cards
### A Rust library for playing cards and poker hand evaluation

Playing Cards is a Rust library that implements playing cards, primarily focussing on hand evaluation for games like poker.

## Features

### Core

Here is a list of types included within the Core package:

- Card
- Carddeck

### Poker

Here is a list of type included within the Poker feature:

- Evaluators
  - High Evaluator
  - Low Evaluator (TODO)
  - Omaha Evaluator (TODO)
  - Omaha Hi-Lo Evaluator (TODO)
  - Drawmaha Evaluator (TODO)
  - Drawmaha 0 Evaluator (TODO)
  - Drawmaha 49 Evaluator (TODO)
  - Shortdeck High Evaluator (TODO)

All the evaluators run on the ever-popular [2+2 evalautor](https://github.com/tangentforks/XPokerEval/tree/master/XPokerEval.TwoPlusTwo),
which can evaluate 5 card hands from a given set of 5-7 cards.

## Other Things Still TODO

- [X] Create build script to generate 2+2 table ~~(dump the struct into a file using Abomination)~~ (use bincode to encode and decode the Vec table)
- [ ] Improve loading the 2+2 table into static memory
  - Issues regarding this involve reading the table for the first time (since that is when
  lazy_static initializes the lookup table)
  - Right now bincode is a safe approach, but might reconsider to use Abomination if it significantly
  reduces lazy static runtime.
  - The golden standard would be for the build script to generate a phf Hashmap during compile time,
  however in practice, this makes compile time *unbearably* slow due to the size of the Hashmap that
  gets generated. (Reason is due to the runtime of calcualting perfect hashes.)
- [ ] Build the rest of the evaluators
- [X] Documentation
