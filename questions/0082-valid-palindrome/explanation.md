# Approach: two pointers that skip noise

Keep `left` at the start and `right` at the end. At each step, if the
character under a pointer isn't a letter or digit, step past it. Once both
point at letters or digits, compare their lowercase forms: a mismatch ends the
check, a match moves both pointers inward. When the pointers meet, every pair
has matched.

```python
def is_palindrome(s):
    def keep(ch):
        return ch.isascii() and ch.isalnum()

    left, right = 0, len(s) - 1
    while left < right:
        if not keep(s[left]):
            left += 1
        elif not keep(s[right]):
            right -= 1
        elif s[left].lower() != s[right].lower():
            return False
        else:
            left += 1
            right -= 1
    return True
```

**Alternative:** filter and lowercase the characters into a new list, then
compare it with its reverse. Same time, but O(n) extra space.

## Complexity

- Time: O(n): each pointer moves at most `n` times.
- Space: O(1).

## Pitfalls

- Digits count: `"0P"` is not a palindrome, and `"1b1"` is.
- The underscore `_` is not a letter or digit: `"ab_a"` is a palindrome.
- A text with nothing but punctuation and spaces is a palindrome.
- In JavaScript, a regex such as `/\w/` matches `_`; test for `[A-Za-z0-9]`
  explicitly.
