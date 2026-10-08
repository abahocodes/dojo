A row of people stands in a line, all facing right. `heights[i]` is the height
of the person at position `i`, and all heights are **different**.

Person `i` can see person `j` (with `j > i`) when everyone standing strictly
between them is shorter than **both** person `i` and person `j`. Two people
standing next to each other always see each other.

Return an array `answer` of the same length where `answer[i]` is the number
of people to the right of person `i` that person `i` can see.

## Example 1

```
heights = [8, 3, 5, 2, 9, 4]
output  = [3, 1, 2, 1, 1, 0]
# person 0 (8) sees 3, 5 and 9; the 2 is hidden behind the 5
# person 2 (5) sees 2 and 9
```

## Example 2

```
heights = [1, 2, 3, 4]
output  = [1, 1, 1, 0]   # each person's taller neighbour blocks the rest
```

## Constraints

- `1 <= len(heights) <= 10^5`
- `1 <= heights[i] <= 10^5`
- all values of `heights` are distinct
