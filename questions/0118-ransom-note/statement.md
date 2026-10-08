You want to compose `ransom_note` by cutting single letters out of
`magazine`. Each letter of the magazine can be used **at most once**, and the
order of letters in the magazine does not matter.

Return `true` if the note can be composed, otherwise `false`.

## Example 1

```
ransom_note = "note"
magazine    = "tonedeaf"
output      = true
```

## Example 2

```
ransom_note = "hello"
magazine    = "hole"
output      = false   # the note needs two 'l's, the magazine has one
```

## Constraints

- `1 <= len(ransom_note), len(magazine) <= 10^5`
- Both strings contain only lowercase English letters.
