def tree2str(root: "TreeNode") -> str:
    # the stack holds nodes still to print and literal text to emit
    parts = []
    stack = [root]
    while stack:
        item = stack.pop()
        if isinstance(item, str):
            parts.append(item)
            continue
        parts.append(str(item.val))
        # pushed in reverse: left group first, then right group
        if item.right:
            stack.extend([")", item.right, "("])
        if item.left:
            stack.extend([")", item.left, "("])
        elif item.right:
            stack.append("()")
    return "".join(parts)
