# Hints

## Hint 1
A hash map from key to its history is the obvious start. Scanning a key's
whole history on every `get` works, but it is O(number of writes) per read.
What do you know about the order in which a key's entries arrive?

## Hint 2
`set` timestamps only increase, so appending each `(timestamp, value)` to the
key's list keeps that list sorted by time, for free.

## Hint 3
For `get(key, t)`, binary-search the key's sorted list for the **last** entry
with timestamp `<= t`. If there is none (every write came later, or the key is
missing) return `""`; otherwise return that entry's value.
