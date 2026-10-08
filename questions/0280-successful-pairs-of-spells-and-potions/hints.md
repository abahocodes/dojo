# Hints

## Hint 1
Checking every spell against every potion is up to `10^10` products. For a
fixed spell `s`, which potions work? Is there a threshold strength?

## Hint 2
A potion `p` works with spell `s` exactly when `p >= ceil(success / s)`. If
the potions were sorted, the working ones would form a suffix.

## Hint 3
Sort `potions` once. For each spell compute `need = ceil(success / s)` with
integer arithmetic, binary search for the first potion `>= need`, and the
count is `len(potions)` minus that index.
