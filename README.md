Graphael
========
## What is Graphael?
Graphael is a [graph database](http://en.wikipedia.org/wiki/Graph_database) written in Rust.

## Dependencies

Graphael is built on Rust 1.5.0

The easiest way to install Rust is through the installers provided on [the Rust install page](http://www.rust-lang.org/install.html).

*Please Note* This version of Graphael requires the nightly version of Rust.

## Getting Started

1. Run `cargo build` in the root directory
2. Run `cargo run` to run the program
## Examples
```
Enter a graph name> langs
Using 'langs'
1. Nodes with attribute.
2. Nodes with key-value.
3. Edges with label.
4. Edges from node with label.
5. Look up node.

>>> 1
Enter an attribute> ProgrammingLangauge
[Node<id=245, props={name: Alef}>, Node<id=2, props={name: Clipper}>, Node<id=5, props={name: KornShell}> ...]

>>> 2
Enter key-value> name = C
[Node<id=100, props={name: C}>]

>>> 3
Enter label> influenced
{35: {213: Edge<labels=[influenced]>}, 229: {33: Edge<labels=[influenced]>}, 249: {54: Edge<labels=[influenced]> ...}

>>> 4
Enter node and label> 100 HAS influenced
[Node<id=87, props={name: BitC}>, Node<id=158, props={name: Pike}>, Node<id=114, props={name: D programming language}>, Node<id=107, props={name: Go}>, Node<id=28, props={name: AWK}>, Node<id=71, props={name: Perl}>, ...]

>>> 5
Enter node id> 100
Some(Node<id=100, props={name: C}>)
```
