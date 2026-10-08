def total_strength(strength: list[int]) -> int:
    MOD = 10**9 + 7
    n = len(strength)
    left, right = [-1] * n, [n] * n
    stack = []
    for i, x in enumerate(strength):
        while stack and strength[stack[-1]] >= x:
            right[stack.pop()] = i  # first smaller-or-equal to the right
        left[i] = stack[-1] if stack else -1  # first strictly smaller to the left
        stack.append(i)
    # pp[k] = P[0] + ... + P[k-1], where P[k] = strength[0] + ... + strength[k-1]
    pp = [0] * (n + 2)
    p = 0
    for k in range(n + 1):
        pp[k + 1] = (pp[k] + p) % MOD
        if k < n:
            p = (p + strength[k]) % MOD
    total = 0
    for i, x in enumerate(strength):
        l, r = left[i], right[i]
        plus = (i - l) * (pp[r + 1] - pp[i + 1])
        minus = (r - i) * (pp[i + 1] - pp[l + 1])
        total = (total + x * ((plus - minus) % MOD)) % MOD
    return total
