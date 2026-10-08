# Hints

## Hint 1
Fix the end of the substring. If the substring `s[i..j]` contains all three
letters, does `s[i-1..j]`?

## Hint 2
Yes: extending to the left never loses a letter. So for each end `j`, the
valid starts form a prefix `0..m` of positions. How do you find `m` quickly?

## Hint 3
Track the last index where each letter appeared. A start `i` works exactly when
`i <= min(last_a, last_b, last_c)`, so add `min(...) + 1` for each `j` (adding
0 while some letter has not appeared yet).
