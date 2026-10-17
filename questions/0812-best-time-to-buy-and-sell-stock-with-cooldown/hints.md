# Hints

## Hint 1
Without the cooldown you would grab every price increase. The cooldown
couples neighbouring trades, so a greedy rule no longer works. Try
describing where you are at the end of each day instead.

## Hint 2
At the end of a day you are in one of three states: **holding** a share,
**just sold** today, or **resting** (no share, free to buy tomorrow). For
each state, track the best profit you can have reached it with.

## Hint 3
Transitions for price `p`:
`hold' = max(hold, rest - p)` (keep holding, or buy from rest),
`sold' = hold + p` (sell today),
`rest' = max(rest, sold)` (stay idle, or yesterday's sale cools down).
Start with `hold = -∞`, `sold = rest = 0`; the answer is `max(sold, rest)`.
