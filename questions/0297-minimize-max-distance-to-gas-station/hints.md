# Hints

## Hint 1
Fix a target `D`. How many new stations does a single gap of length `g` need so
that every piece is at most `D` long?

## Hint 2
A gap of length `g` needs `ceil(g / D) - 1` new stations. Summing over all gaps
tells you whether `D` is reachable with `k` stations, and a larger `D` never
needs more stations.

## Hint 3
Binary search `D` over real numbers between 0 and the largest gap. Running a
fixed number of halvings (say 100) gives full double precision without
worrying about a stopping epsilon.
