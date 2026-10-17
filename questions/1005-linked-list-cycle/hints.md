# Hints

## Hint 1
If there is no cycle, walking `next` reaches `None`. If there is one, the walk
never ends, so you need a way to notice that you are going around. Note that
values repeat: seeing a value twice proves nothing.

## Hint 2
Remember the nodes themselves (not their values) in a hash set as you walk.
Meeting a node that's already in the set means there is a cycle. That is O(n)
time and O(n) memory.

## Hint 3
For O(1) memory, move two pointers from `head`: `slow` one step at a time,
`fast` two steps. If `fast` reaches the end, there's no cycle. Inside a cycle
the gap between them shrinks by one every step, so they must meet.
