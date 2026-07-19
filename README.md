# rust-data-structures

An educational Rust library implementing classic data structures from first principles.

The goal of this project is not simply to provide working implementations, but to demonstrate idiomatic Rust through clean design, thorough documentation, comprehensive tests, and thoughtful discussion of implementation tradeoffs.

## Philosophy

This library is built with the following principles:

* Write clear, idiomatic Rust.
* Favor readability over cleverness.
* Explain *why* a design was chosen, not just *how* it works.
* Document ownership and borrowing decisions.
* Include comprehensive unit and integration tests.
* Benchmark when performance matters.
* Build each data structure as if it were a production-quality library.

This project is intended to be a learning resource for Rust programmers of all experience levels.

## Current Status

The library is under active development.

### Planned Data Structures

* Binary Search Tree
* AVL Tree
* Red-Black Tree
* Binary Heap
* Singly Linked List
* Doubly Linked List
* Trie
* Fenwick Tree (Binary Indexed Tree)
* Segment Tree
* Disjoint Set Union (Union-Find)
* Graph Representations
* Additional structures as the project evolves

## Project Structure

```text
src/
├── bst/
├── heap/
├── linked_list/
├── trie/
├── graph/
└── ...
```

Each data structure is implemented as an independent module with its own documentation and tests.

## Design Goals

Every module aims to answer the following questions:

* When should this data structure be used?
* What operations is it optimized for?
* What are its time and space complexities?
* How does Rust's ownership model influence the implementation?
* What tradeoffs were made during the design?

## Testing

The project includes:

* Unit tests
* Integration tests
* Documentation examples (doctests)

All code should pass:

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features
```

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0). See the `LICENSE` file for details.

## Author

**Michael N. Rowsey**

## Contributing

Contributions, suggestions, bug reports, and discussions are welcome.

If you have ideas for improving an implementation, documentation, or educational examples, please open an issue or submit a pull request.

