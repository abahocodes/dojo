# Hints

## Hint 1
Look at one attack and the attack right after it. How much of the first
attack's poison actually gets used before the second one lands?

## Hint 2
If the gap to the next attack is at least `duration`, the first poison runs
its full course. Otherwise only `gap` seconds of it count before the restart.

## Hint 3
Sum `min(duration, time_series[i + 1] - time_series[i])` over consecutive
pairs, then add `duration` for the last attack, which is never cut short.
