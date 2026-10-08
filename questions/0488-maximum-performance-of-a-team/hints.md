# Hints

## Hint 1
Two quantities pull in different directions: the speed sum and the minimum
efficiency. Fix one of them. If you decide which engineer has the smallest
efficiency in the team, what is the best way to fill the remaining places?

## Hint 2
With engineer `i` as the efficiency minimum, every teammate must have
efficiency at least `efficiency[i]`, and you should take the `k - 1` fastest
of them. Sorting engineers by efficiency from highest to lowest makes the
eligible teammates exactly the ones you have already seen.

## Hint 3
Scan in descending efficiency, keeping the `k` largest speeds seen so far in
a min-heap with their running sum (pop the smallest when it exceeds `k`).
After adding engineer `i`, evaluate `sum * efficiency[i]`. Keep the maximum in
a 64-bit integer and apply the modulus only at the end.
