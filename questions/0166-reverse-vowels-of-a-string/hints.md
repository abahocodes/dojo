# Hints

## Hint 1
Collect the vowels, reverse them, and write them back into the vowel slots.
That works; can you do it in a single pass without the extra list?

## Hint 2
The first vowel swaps with the last vowel, the second with the second-to-last,
and so on.

## Hint 3
Use two pointers starting at both ends. Move the left one right past
non-vowels and the right one left past non-vowels; when both sit on vowels,
swap them and move both inward.
