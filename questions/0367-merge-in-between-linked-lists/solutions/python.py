def merge_in_between(list1, a, b, list2):
    before = list1
    for _ in range(a - 1):
        before = before.next
    after = before
    for _ in range(b - a + 2):
        after = after.next
    before.next = list2
    tail = list2
    while tail.next:
        tail = tail.next
    tail.next = after
    return list1
