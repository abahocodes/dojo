# Hints

## Hint 1
Checking "is this value in `nums`?" for every node should be O(1). Put `nums`
into a set (or a boolean array indexed by value).

## Hint 2
Each component has exactly one last node. What distinguishes the last node of
a component from the others?

## Hint 3
Walk the list once and count nodes whose value is in the set and whose `next`
is either null or has a value not in the set.
