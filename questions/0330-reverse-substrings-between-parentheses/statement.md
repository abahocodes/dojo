You are given a string `s` made of lowercase letters and parentheses, where the
parentheses are balanced (every `"("` has a matching `")"` after it, and pairs
nest properly).

Process the pairs from the innermost outwards: for each matching pair, reverse
the text currently between them. Return the final text with every parenthesis
removed.

## Example 1

```
s = "(ab(cd)e)"
output = "ecdba"
```

The inner pair turns `"cd"` into `"dc"`, giving `"(abdce)"`; reversing the
outer pair gives `"ecdba"`.

## Example 2

```
s = "x(yz)w"
output = "xzyw"
```

## Constraints

- `1 <= len(s) <= 2000`
- `s` contains only lowercase English letters, `"("` and `")"`.
- The parentheses in `s` are balanced. The result may be empty.
