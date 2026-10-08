# Hints

## Hint 1
Pick any window of `s`. To make it a single repeated letter cheaply, you keep
its most frequent letter and overwrite everything else. How many edits does that
cost?

## Hint 2
A window is achievable when `window_length - (count of its most frequent letter) <= k`.
Slide a window over `s`, growing on the right and shrinking from the left when
the condition breaks.

## Hint 3
Track letter counts in the window and the largest count ever seen, `max_freq`.
You never need to decrease `max_freq` when shrinking: a smaller value can never
produce a longer answer than one you have already recorded, so the window simply
stops growing until a letter beats the old record.
