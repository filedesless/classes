# SVP

experimentation on the Shortest Vector Problem in lattices

## Ideas

- implement LLL basis reduction (SVP approximation)
- cut branches whenever new minimum found by recomputing bounds (involves iterating in a spiral around 0)
- batch point enumeration n by n to benefit from faster matrix multiplication maybe?
- use ideal lower bound to skip uneeded norm calculations