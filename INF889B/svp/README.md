# SVP

experimentation on the Shortest Vector Problem in lattices

## TODO

- cut branches whenever new minimum found by recomputing bounds (involves iterating in a spiral around 0)
- use ideal lower bound to skip uneeded norm calculations
- batch point enumeration n by n to benefit from faster matrix multiplication maybe?

## TODONE

- implement LLL basis reduction (SVP approximation)
- cut search space in half by respecting ||v|| = ||-v||