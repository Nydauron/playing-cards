# Contributing

Thanks for expressing interest in contributing to the playing-cards library! Before you begin making
your changes, let's go over some guidelines.

## Submitting bug reports and feature requests

When reporting bugs, please include a minimal reproducible example, providing as many details needed
such that others and I can reproduce the behavior you are experiencing. Please also include the
expected result vs the actual result if applicable.

When suggesting a new feature, please explain your reasoning as to why the feature should be added,
what potential problems it solves, any alternatives, and any disadvantanges.

## Submitting a PR

1. Fork the repo and make your changes
2. Create a new PR
  - Please describe in detail what your PR changes including but not limited to any feature addtions,
  bug fixes, and optimizations.
  - Your PR should add and modify documentation and the README such that it remains consistent with
  any feature changes or bug fixes made in the PR.
3. All PRs must pass `cargo clippy` and `cargo fmt` which is checked by CI. No warnings
are allowed on compilation.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
