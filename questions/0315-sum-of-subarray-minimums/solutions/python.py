def sum_subarray_mins(arr: list[int]) -> int:
    MOD = 10**9 + 7
    n = len(arr)
    stack = []
    total = 0
    for j in range(n + 1):
        cur = arr[j] if j < n else 0
        while stack and arr[stack[-1]] >= cur:
            i = stack.pop()
            left = stack[-1] if stack else -1
            total += arr[i] * (i - left) * (j - i)
        stack.append(j)
    return total % MOD
