# Hints

## Hint 1
Counting every value in a hash map and picking the one above `n / 2` works in
O(n) time and O(n) memory. Sorting also works: which index of the sorted list
must hold the majority value?

## Hint 2
For O(1) memory: imagine pairing up each occurrence of the majority value with
a different value and cancelling both. Since the majority value is more than
half the list, what is left over at the end?

## Hint 3
Keep a `candidate` and a `count`. When `count` is `0`, adopt the current value
as the candidate. Then add 1 if the value equals the candidate and subtract 1
otherwise. After one pass, `candidate` is the majority value.
