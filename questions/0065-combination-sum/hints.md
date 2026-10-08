# Hints

## Hint 1
Try building combinations by picking values one at a time and subtracting
them from the target. When the remainder reaches `0` you have a combination;
when it goes negative you can stop.

## Hint 2
Naive branching produces `[2, 3, 3]`, `[3, 2, 3]` and `[3, 3, 2]` separately.
How can you force every combination to be built in exactly one order?

## Hint 3
Only ever pick candidates at index `i` or later, where `i` is the index of the
last candidate you picked (passing `i`, not `i + 1`, allows reuse). Sorting the
candidates first lets you stop the loop as soon as one is larger than the
remainder.
