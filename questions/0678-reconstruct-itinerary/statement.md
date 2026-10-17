You found a pile of used plane tickets. Each ticket `[from, to]` is a
one-way flight between two airports, given by three-letter codes. All the
tickets belong to one traveller, who started at `"JFK"` and used **every**
ticket exactly once.

Rebuild the trip as the list of airports visited in order, starting with
`"JFK"`. Several orders may use every ticket; return the one that is
smallest in dictionary order when the list is read as a sequence of codes
(compare the first airport, then the second, and so on). For example,
`["JFK", "LGA"]` comes before `["JFK", "LGB"]`.

The input always admits at least one valid trip. The same flight may appear
on several tickets, and each copy must be used once.

## Example 1

```
tickets = [["SFO", "LAX"], ["JFK", "ATL"], ["ATL", "SFO"],
           ["LAX", "JFK"], ["JFK", "SFO"]]
output  = ["JFK", "ATL", "SFO", "LAX", "JFK", "SFO"]
```

From `JFK` you can fly to `ATL` or `SFO`; `ATL` is smaller, and the trip
can still use every ticket after taking it.

## Example 2

```
tickets = [["JFK", "KUL"], ["JFK", "NRT"], ["NRT", "JFK"]]
output  = ["JFK", "NRT", "JFK", "KUL"]
```

`KUL` is smaller than `NRT`, but flying there first strands you: there is
no ticket out of `KUL`, and the other two tickets would go unused. So the
smallest **valid** trip goes to `NRT` first.

## Constraints

- `1 <= len(tickets) <= 300`
- Every ticket has two entries, each of three uppercase English letters.
- `from != to` on every ticket.
- At least one trip starting at `"JFK"` uses every ticket exactly once.
