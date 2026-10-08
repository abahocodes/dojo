# Hints

## Hint 1
Trying every start and end is O(n²). Instead, ask: what is the best total of a
stretch that **ends exactly at** index `i`?

## Hint 2
The best stretch ending at `i` either extends the best stretch ending at `i - 1`,
or starts fresh at `i`. When is starting fresh the better choice?

## Hint 3
Keep `current = max(x, current + x)` as you walk the list, and track the largest
`current` you ever see. Start both from `nums[0]`, not from 0, so all-negative
inputs work.
