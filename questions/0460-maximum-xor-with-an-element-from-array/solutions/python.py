def maximize_xor(nums: list[int], queries: list[list[int]]) -> list[int]:
    BITS = 30
    nums = sorted(nums)
    order = sorted(range(len(queries)), key=lambda i: queries[i][1])
    zero = [0]  # child index for bit 0, per node
    one = [0]   # child index for bit 1, per node
    answer = [-1] * len(queries)
    j = 0

    for qi in order:
        x, limit = queries[qi]
        while j < len(nums) and nums[j] <= limit:
            v = nums[j]
            node = 0
            for b in range(BITS - 1, -1, -1):
                children = one if (v >> b) & 1 else zero
                if children[node] == 0:
                    children[node] = len(zero)
                    zero.append(0)
                    one.append(0)
                node = children[node]
            j += 1
        if j == 0:
            continue
        node = 0
        best = 0
        for b in range(BITS - 1, -1, -1):
            if (x >> b) & 1:
                preferred, other = zero, one
            else:
                preferred, other = one, zero
            if preferred[node]:
                best |= 1 << b
                node = preferred[node]
            else:
                node = other[node]
        answer[qi] = best

    return answer
