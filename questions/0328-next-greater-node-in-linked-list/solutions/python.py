def next_larger_nodes(head):
    vals = []
    while head:
        vals.append(head.val)
        head = head.next
    answer = [0] * len(vals)
    waiting = []
    for i, v in enumerate(vals):
        while waiting and vals[waiting[-1]] < v:
            answer[waiting.pop()] = v
        waiting.append(i)
    return answer
