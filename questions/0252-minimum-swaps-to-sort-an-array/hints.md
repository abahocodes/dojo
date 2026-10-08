# Hints

## Hint 1
Sort a copy to learn where every value has to end up. Now each position
"points" at the position its current value belongs in.

## Hint 2
Follow those pointers from any position and you eventually come back to where
you started. The positions split into disjoint cycles.

## Hint 3
A cycle of length `L` can be fixed with `L - 1` exchanges (put one value in
place each time; the last exchange places two), and never fewer. Sum `L - 1`
over all cycles.
