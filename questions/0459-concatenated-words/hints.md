# Hints

## Hint 1
For a single word this is word break: can it be cut into pieces that are all
dictionary words? The only twist is that the word must not use itself.

## Hint 2
Any piece of a concatenated word is strictly shorter than the word. Process
the words from shortest to longest and only ever look up words processed
earlier.

## Hint 3
Keep a set of the words seen so far. For each word (in length order), run the
word-break DP `can[end] = any(can[start] and w[start:end] in seen)`, record
the word if `can[len(w)]` is true, then add it to the set.
