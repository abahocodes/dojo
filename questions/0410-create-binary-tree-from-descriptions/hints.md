# Hints

## Hint 1
A value may show up as a child before it shows up as a parent (or the other
way round). How can you make sure each value maps to one single node object,
whichever order the triples arrive in?

## Hint 2
Keep a hash map from value to node, creating a node the first time a value is
seen. Then every triple is a single pointer assignment:
`nodes[parent].left = nodes[child]` (or `.right`).

## Hint 3
Which value is the root? It's the only one that never appears in the `child`
position. Record every child in a set while linking, then return the parent
that's missing from that set.
