# Hints

## Hint 1
Trying every (buy, sell) pair is O(n²). If you fix the day you sell, which buy
day is the best one?

## Hint 2
For a fixed sell day, the best buy is the cheapest price seen on any earlier
day. You can maintain that minimum as you scan.

## Hint 3
Walk the prices once, keeping `lowest` so far. At each price, update the answer
with `price - lowest`, then update `lowest`. Start the answer at 0.
