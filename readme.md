# Rust Library cheatsheet:
* petgraph
    * petgraph::algo::dijstra - find shortest distance
    * petgraph::algo::kosaraju_scc - strongly detected components in graph

* rasciigraph  - plotting graph in ascii


* rand = "0.8.5"
* clap - command line arg parser -https://docs.rs/clap/latest/clap/_tutorial/chapter_2/index.html



# Other notes:

* Stop cargo creating repo for crates in project: cargo new my_project --vcs none


# Data structs:

* BTreeSet
  * sorted
  * unique (set!)
* LinkedList
* Hashmap (dict in python)
* Vec (like list in python, mutable)
  * .windows(2) // gives window of n elements to iterate through
* binaryheap = binary tree but it is not represented in the traditional pointer-based tree structure commonly used in data structures. Instead, it is typically implemented using an array-like structure to optimize memory usage and access patterns.