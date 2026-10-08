def remove_nodes(head):
    stack = []
    node = head
    while node:
        while stack and stack[-1].val < node.val:
            stack.pop()
        stack.append(node)
        node = node.next
    for i in range(len(stack) - 1):
        stack[i].next = stack[i + 1]
    stack[-1].next = None
    return stack[0]
