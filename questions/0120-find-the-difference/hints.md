# Hints

## Hint 1
Because `t` is shuffled, positions tell you nothing. Think about what is the
same and what differs in the letter counts.

## Hint 2
Exactly one letter has a count in `t` that is one higher than in `s`. Counting
both strings finds it.

## Hint 3
You don't even need counts: XOR the codes of every character in both strings.
Each letter of `s` appears twice and cancels out, leaving the extra one.
