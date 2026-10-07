# Hints

## Hint 1
Checking every substring is O(n²) or worse. Suppose you know the best window
that ends at position `i - 1`. How does it change when you add `s[i]`?

## Hint 2
Keep a window `[left, i]` with all-distinct characters. When `s[i]` is already
inside the window, the window's left edge must jump past the earlier copy.

## Hint 3
Store the last index where each character was seen. On `s[i]`, if its last
index is `>= left`, set `left = last[s[i]] + 1`. Then record `last[s[i]] = i`
and update the answer with `i - left + 1`.
