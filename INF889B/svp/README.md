# SVP

experimentation on the Shortest Vector Problem in lattices

## TODO

- use ideal lower bound to skip uneeded norm calculations

## TODONE

- implement LLL basis reduction (SVP approximation)
- cut search space in half by respecting ||v|| = ||-v||
- cut branches whenever new minimum found by recomputing bounds (involves iterating in a spiral around 0)

## TODON'T

- batch point enumeration n by n to benefit from faster matrix multiplication maybe?
    - performance regressions observed for smaller bases (n <= 10)