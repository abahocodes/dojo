# Approach: min-heap (priority queue)

Maintain a min-heap whose entries are the current head nodes of the k lists. At every step the heap top is the globally smallest remaining node, so we can build the merged list in one pass.

1. Push the head of every non-empty list into the heap.
2. While the heap is non-empty, pop the smallest node, append it to the result, and if that node has a `next`, push `next` into the heap.
3. Return the head of the result.

Each of the N total nodes is pushed and popped once, and each heap operation costs O(log k), so the total time is **O(N log k)**.

```python
import heapq

def merge_k_sorted_lists(lists):
    if not lists:
        return None
    heap = []
    counter = 0  # tie-breaker so Python never compares ListNode objects
    for node in lists:
        if node is not None:
            heapq.heappush(heap, (node.val, counter, node))
            counter += 1

    dummy = ListNode(0)
    curr = dummy
    while heap:
        _, _, node = heapq.heappop(heap)
        curr.next = node
        curr = curr.next
        if node.next is not None:
            heapq.heappush(heap, (node.next.val, counter, node.next))
            counter += 1
    return dummy.next
```

## Alternative: divide-and-conquer

Pair up the lists and merge each pair, halving the number of lists each round. This also gives O(N log k) time but with a smaller constant and no heap overhead.

## Complexity

- **Time:** O(N log k) where N is the total number of nodes and k is the number of lists.
- **Space:** O(k) for the heap (at most k nodes at any time).

## Pitfalls

- **Tie-breaking in the heap:** in Python, if two nodes have the same `val`, `heapq` will try to compare the `ListNode` objects, which raises a `TypeError`. Use a monotonically increasing counter as a second key to guarantee a total order.
- **Null / empty lists:** guard every push with an `if node is not None:` check so that `null` entries in the input array are skipped.
- **Empty input:** if `lists` is `None` or `[]`, return `None` immediately.
- **Off-by-one in the result:** use a dummy head node to avoid special-casing the first node of the merged list.
