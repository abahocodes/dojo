# Approach: two pointers swapping vowels

Reversing the vowel sequence means swapping the first vowel with the last, the
second with the second-to-last, and so on. Two pointers find those pairs:
`lo` advances past non-vowels, `hi` retreats past non-vowels, and when both
point at vowels they swap and step inward.

```python
def reverse_vowels(s: str) -> str:
    vowels = set("aeiouAEIOU")
    chars = list(s)
    lo, hi = 0, len(chars) - 1
    while lo < hi:
        if chars[lo] not in vowels:
            lo += 1
        elif chars[hi] not in vowels:
            hi -= 1
        else:
            chars[lo], chars[hi] = chars[hi], chars[lo]
            lo += 1
            hi -= 1
    return "".join(chars)
```

## Complexity

- Time: O(n): each pointer crosses the string at most once.
- Space: O(n) for the mutable copy of the string (strings are immutable in
  most of these languages); O(1) beyond that.

## Pitfalls

- Forgetting uppercase vowels.
- Treating `y` as a vowel. It is not one here.
- Building the result with repeated string concatenation, which can be
  quadratic.
