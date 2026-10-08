import heapq

def merge_k_sorted_lists(lists) -> ListNode:
    if lists is None or len(lists) == 0:
        return None

    heap = []
    counter = 0
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
