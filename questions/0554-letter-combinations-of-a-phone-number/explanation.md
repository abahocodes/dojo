# Approach: backtracking, one letter per digit

Every answer is built by choosing one letter for digit 0, then one for
digit 1, and so on. A recursive function plays the role of a variable number
of nested loops: `backtrack(i)` loops over the letters of `digits[i]`,
appends one to the current word and recurses on `i + 1`. When `i` reaches
the end, the word is complete.

```python
KEYS = {"2": "abc", "3": "def", "4": "ghi", "5": "jkl",
        "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz"}

def letter_combinations(digits):
    if not digits:
        return []
    result, current = [], []

    def backtrack(i):
        if i == len(digits):
            result.append("".join(current))
            return
        for letter in KEYS[digits[i]]:
            current.append(letter)
            backtrack(i + 1)
            current.pop()

    backtrack(0)
    return result
```

**Alternative (iterative):** start with `[""]` and, for each digit, replace
the list by every word extended with every letter of that digit. This is
the same product built breadth-first.

## Complexity

- Time: O(4^n · n) for `n` digits: up to `4^n` words (digits `7` and `9`
  have four letters), each of length `n`.
- Space: O(n) recursion depth, plus the output.

## Pitfalls

- Empty input must return `[]`. The plain recursion would return `[""]`,
  one empty word, which is wrong.
- `7` and `9` have four letters, not three; a keypad built by slicing the
  alphabet into groups of three gets everything after `p` wrong.
- Building words by string concatenation in a loop over all words is fine
  here (`n <= 4`), but appending to and popping from one buffer avoids
  creating a new string at every step.
