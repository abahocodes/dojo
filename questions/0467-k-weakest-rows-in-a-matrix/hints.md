# Hints

## Hint 1
First reduce each row to a single number: how many soldiers it has. Then the
problem is "rank rows by that number, breaking ties by index". Because each row
is 1s then 0s, the count is the position of the first 0.

## Hint 2
The row is sorted (descending), so the first 0 can be found by binary search
in O(log cols) instead of scanning the whole row.

## Hint 3
Build pairs `(soldiers, index)` and sort them; tuples compare by soldiers
first and index second, which is exactly the tie-break you need. Return the
indices of the first `k` pairs. (A max-heap of size `k` over the same pairs
works too, if you want O(rows · log k).)
