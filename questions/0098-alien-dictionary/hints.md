# Hints

## Hint 1
What can you learn from two **adjacent** words in the list? Look at the
first position where they differ. Is there anything to learn after that
position?

## Hint 2
Adjacent words give at most one rule each: at the first differing position,
letter `a` of the first word comes before letter `b` of the second. These
rules are edges of a directed graph on the letters, and a valid ordering is a
topological order. Also watch for a longer word placed right before its own
prefix: that's impossible.

## Hint 3
Run Kahn's algorithm, but keep the letters that are ready (in-degree zero) in
a min-heap, so you always output the alphabetically smallest ready letter.
That produces the alphabetically first topological order. If you output fewer
letters than there are, the rules contain a cycle: return `""`.
