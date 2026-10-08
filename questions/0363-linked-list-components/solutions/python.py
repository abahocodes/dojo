def num_components(head, nums):
    wanted = set(nums)
    count = 0
    cur = head
    while cur:
        if cur.val in wanted and (cur.next is None or cur.next.val not in wanted):
            count += 1
        cur = cur.next
    return count
