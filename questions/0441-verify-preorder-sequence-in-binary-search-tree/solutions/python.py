def verify_preorder(preorder: list[int]) -> bool:
    low = float("-inf")
    stack = []
    for x in preorder:
        if x < low:
            return False
        while stack and stack[-1] < x:
            low = stack.pop()
        stack.append(x)
    return True
