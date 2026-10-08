# Hints

## Hint 1
Comparing every pair is O(n^2), too slow for 40,000 dominoes. Group
equivalent dominoes instead.

## Hint 2
Give each domino a canonical form that is the same for both rotations, for
example the pair with the smaller value first.

## Hint 3
Walk the list and keep a count per canonical form. A new domino forms a pair
with every earlier domino of the same form, so add the current count before
incrementing it. With values 1..9, `10 * min + max` is a fine array index.
