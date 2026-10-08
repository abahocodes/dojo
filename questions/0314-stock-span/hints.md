# Hints

## Hint 1
The span of day `i` is `i - j`, where `j` is the closest earlier day with a
price strictly greater than `prices[i]` (or `-1` if there is none).

## Hint 2
Once a higher-or-equal price appears on day `i`, no later day will ever stop
at an earlier, lower-or-equal day: day `i` blocks its view first or is
passed over along with it.

## Hint 3
Keep a stack of day indices with strictly decreasing prices. For each new day,
pop every index whose price is `<=` today's; the index left on top (if any) is
the blocking day `j`. Push today and move on.
