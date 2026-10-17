# Hints

## Hint 1
A hash map gives O(1) lookups, but it can't tell you which key was used
longest ago. Keeping keys in a list ordered by last use gives you that, but
moving a key to the front of an array is O(n). Which list structure lets you
unlink an element from the middle in O(1)?

## Hint 2
Use a doubly linked list ordered by recency (most recent at one end, least
recent at the other) **and** a hash map from key to the list node holding it.
The map finds a node in O(1); the node's `prev`/`next` pointers let you
unlink it in O(1).

## Hint 3
`get`: look the node up; if it exists, move it to the "recent" end and return
its value. `put`: if the key exists, update the value and move the node;
otherwise create a node at the "recent" end, add it to the map, and if the
size now exceeds `capacity`, remove the node at the "old" end from both the
list and the map. Dummy head and tail nodes remove all the empty-list
special cases.
