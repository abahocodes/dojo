# Hints

## Hint 1
If you could turn each subtree into a key that is equal exactly when the
subtrees are equal, a hash map from key to count would solve the problem.

## Hint 2
A key can be built bottom-up: a subtree is described by its left subtree's key,
its value and its right subtree's key. Visit children before parents
(postorder). Mark empty children explicitly, or different shapes such as
"1 with left child 2" and "1 with right child 2" collide.

## Hint 3
Full serialized strings can grow to O(n) characters each, which makes the
whole thing O(n^2). Instead give every distinct `(left id, value, right id)`
triple a small integer id (0 for an empty child). Count how often each id
appears and record a node the moment its id's count reaches exactly 2.
