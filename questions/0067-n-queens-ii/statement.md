A chess queen attacks every square in its row, its column and both of its
diagonals. Given `n`, count the ways to place `n` queens on an `n × n` board
so that no queen attacks another.

Two placements are different if some square holds a queen in one but not the
other. Rotations and reflections of a placement count separately.

## Example 1

```
n      = 4
output = 2
# . Q . .      . . Q .
# . . . Q      Q . . .
# Q . . .      . . . Q
# . . Q .      . Q . .
```

## Example 2

```
n      = 1
output = 1
```

## Constraints

- `1 <= n <= 10`
