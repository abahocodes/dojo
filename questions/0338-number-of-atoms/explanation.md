# Approach: stack of count maps

Scan the formula once with an index `i` and a stack of dictionaries; the
bottom dictionary is the whole formula.

- `'('`: push an empty dictionary for the group.
- `')'`: read the digits after it (default `1`), pop the group's dictionary
  and add each of its counts, multiplied, into the dictionary below.
- An uppercase letter: read the following lowercase letters as the element
  name, then the digits as its count (default `1`), and add it to the top
  dictionary.

When the scan ends, sort the element names and concatenate each name with its
count, skipping counts of `1`.

```python
def count_of_atoms(formula):
    n = len(formula)
    stack = [{}]
    i = 0

    def read_number():
        nonlocal i
        start = i
        while i < n and formula[i].isdigit():
            i += 1
        return int(formula[start:i]) if i > start else 1

    while i < n:
        ch = formula[i]
        if ch == "(":
            stack.append({})
            i += 1
        elif ch == ")":
            i += 1
            mult = read_number()
            group = stack.pop()
            top = stack[-1]
            for name, cnt in group.items():
                top[name] = top.get(name, 0) + cnt * mult
        else:
            start = i
            i += 1
            while i < n and formula[i].islower():
                i += 1
            name = formula[start:i]
            stack[-1][name] = stack[-1].get(name, 0) + read_number()

    counts = stack[0]
    return "".join(name + (str(counts[name]) if counts[name] > 1 else "")
                   for name in sorted(counts))
```

## Complexity

- Time: O(n * d + k log k), where `d` is the nesting depth (a closing
  parenthesis merges its group's map into the parent) and `k` the number of
  distinct elements. In practice this is close to linear.
- Space: O(n) for the maps on the stack.

## Pitfalls

- Reading only one digit: counts and multipliers can have several digits.
- Reading only one letter of an element name: `Mg` is one element, not `M`
  and `g`.
- Printing a count of `1`: the answer writes `H`, never `H1`.
- Recursing per parenthesis is fine in principle, but deep nesting (hundreds
  of levels) can hit recursion limits; the explicit stack avoids that.
- Sorting with a case-insensitive or locale-aware comparison. Plain character
  order is what is asked for.
