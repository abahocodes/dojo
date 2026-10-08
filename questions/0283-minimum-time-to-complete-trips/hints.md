# Hints

## Hint 1
For a fixed time `t`, you can count the trips finished so far in one pass.
How does that count change as `t` grows?

## Hint 2
The count `sum(t // time[i])` never decreases as `t` increases, so you can
binary search for the first `t` where it reaches `total_trips`.

## Hint 3
The fastest bus alone finishes `total_trips` trips by
`min(time) * total_trips`, which is a valid upper bound (up to `10^14`, so use
64-bit integers). While counting, stop as soon as the total reaches
`total_trips` so the sum cannot overflow.
