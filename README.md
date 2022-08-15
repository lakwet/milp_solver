# MILP Solver

Mixed Integer Linear Program Solver

In pure Rust (No non Rust dependency)

**Development in progress.**

The idea is to have a MILP solver in pure Rust. I started by implementing a LP
solver, then I will do the IP solver, then the MILP solver. At last I will
implement the solver for sparse matrix. Once everything is done I will start to
compare different algorithms and try to improve something if I can. The very
goal is to have a state of the art MILP solver in pure Rust.

# (LP) Linear Program solver

I started with Chvatal algorithm. The one that can be found in _Introduction to
algorithms_ 3rd edition, from Cormen, Leiserson, Rivest and Stein.

## How to use it

- [See the documentation](https://docs.rs/milp_solver/)

# (IP) Integer Program solver

TBD

# (MILP) Mixed Integer Linear Program solver

TBD

# Solver for sparse matrix

TBD

# License

This project is under the **Apache 2.0** license.

See the license file.

# References

- [Introduction to algorithms. Cormen, Leiserson, Rivest, Stein](https://edutechlearners.com/download/Introduction_to_algorithms-3rd%20Edition.pdf)
- [Linear Programming: Foundations and Extensions](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.111.1824&rep=rep1&type=pdf)
- [Linear Programming](http://www.dblab.ntua.gr/~gtsat/collection/Karloff_LinearProgramming.pdf)
