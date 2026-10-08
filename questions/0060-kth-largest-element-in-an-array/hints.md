# Hints

## Hint 1
Sorting and indexing works. But you only care about the `k` biggest values;
the order among the rest never matters.

## Hint 2
Suppose you keep the `k` largest values seen so far in some container. Which
one of them is the answer once you have seen the whole list, and which one do
you throw out when a bigger value arrives?

## Hint 3
Keep a **min**-heap of size `k`. For each value, if the heap holds fewer than
`k` items push it; otherwise, if it beats the heap's minimum, replace the
minimum. At the end the heap's top is the `k`-th largest.
