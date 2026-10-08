# Hints

## Hint 1
Deleting the fewest intervals is the same as keeping the most intervals that
don't overlap each other.

## Hint 2
Picture choosing intervals left to right. Among all the intervals you could
pick first, which one leaves the most room for everything after it?

## Hint 3
Sort by **end** time. Greedily keep an interval whenever its start is `>=` the
end of the last kept interval; otherwise delete it. The answer is the number
deleted.
