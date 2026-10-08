You have `n` computers and a pile of batteries; `batteries[i]` is how many
minutes battery `i` can supply. At any moment each computer must be powered by
exactly one battery, and a battery can power at most one computer at a time.

At any whole minute you may unplug batteries and plug in others instantly, as
often as you like. A battery keeps its unused charge when unplugged. Batteries
cannot be recharged.

Return the largest number of whole minutes for which all `n` computers can run
at the same time.

## Example 1

```
n         = 2
batteries = [4, 4, 4]
output    = 6   # swap the third battery between the two computers
```

## Example 2

```
n         = 2
batteries = [9, 1, 1]
output    = 2   # the 9-minute battery can only ever feed one computer;
                # the other runs on the two 1-minute batteries
```

## Constraints

- `1 <= n <= len(batteries) <= 10^5`
- `1 <= batteries[i] <= 10^9`

The answer can exceed `2^31`; return it as a 64-bit integer.
