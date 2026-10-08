def construct_maximum_binary_tree(nums: list[int]) -> "TreeNode":
    # stack holds nodes with strictly decreasing values (the current right spine)
    stack = []
    for x in nums:
        node = TreeNode(x)
        last = None
        while stack and stack[-1].val < x:
            last = stack.pop()
        node.left = last
        if stack:
            stack[-1].right = node
        stack.append(node)
    return stack[0]
