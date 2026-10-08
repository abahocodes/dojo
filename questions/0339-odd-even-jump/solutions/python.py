def odd_even_jumps(arr: list[int]) -> int:
    n = len(arr)

    def targets(order: list[int]) -> list[int]:
        # For each index, the first later entry of `order` that lies to its right.
        nxt = [-1] * n
        stack = []
        for j in order:
            while stack and stack[-1] < j:
                nxt[stack.pop()] = j
            stack.append(j)
        return nxt

    odd_next = targets(sorted(range(n), key=lambda i: (arr[i], i)))
    even_next = targets(sorted(range(n), key=lambda i: (-arr[i], i)))
    odd = [False] * n
    even = [False] * n
    odd[-1] = even[-1] = True
    for i in range(n - 2, -1, -1):
        if odd_next[i] != -1:
            odd[i] = even[odd_next[i]]
        if even_next[i] != -1:
            even[i] = odd[even_next[i]]
    return sum(odd)
