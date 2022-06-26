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

All the evaluators run on the ever-popular [2+2 evalautor](https://github.com/tangentforks/XPokerEval/tree/master/XPokerEval.TwoPlusTwo).

## Other Things Still TODO

- Create build script to generate 2+2 table (dump the struct into a file using Abomimation)
- Improve loading the 2+2 table into static memory
  - Issues regarding this involve reading the table for the first time (since that is when lazy_static initializes the lookup table)
- Build the rest of the evaluators
- Documentation
