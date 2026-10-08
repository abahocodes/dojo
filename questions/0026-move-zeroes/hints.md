# Hints

## Hint 1
Building a new list of the non-zeros and padding it with zeros is easy, but it
uses O(n) extra memory. Can you write the non-zeros into the front of the same
list instead?

## Hint 2
Keep a write position `w`, starting at `0`. Scan the list with a read position;
whenever you read a non-zero value, where should it go, and what happens to `w`?

## Hint 3
Swap `nums[read]` with `nums[w]` whenever `nums[read] != 0`, then advance `w`.
Non-zeros are pulled forward in order and zeros are pushed behind them in a
single pass. Return `nums` at the end.
