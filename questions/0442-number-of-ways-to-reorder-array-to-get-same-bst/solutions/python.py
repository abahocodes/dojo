MOD = 10**9 + 7


def num_of_ways(nums: list[int]) -> int:
    n = len(nums)
    left = [0] * (n + 1)
    right = [0] * (n + 1)
    root = nums[0]
    for v in nums[1:]:
        cur = root
        while True:
            if v < cur:
                if left[cur] == 0:
                    left[cur] = v
                    break
                cur = left[cur]
            else:
                if right[cur] == 0:
                    right[cur] = v
                    break
                cur = right[cur]

    fact = [1] * (n + 1)
    for i in range(1, n + 1):
        fact[i] = fact[i - 1] * i % MOD
    inv_fact = [1] * (n + 1)
    inv_fact[n] = pow(fact[n], MOD - 2, MOD)
    for i in range(n, 0, -1):
        inv_fact[i - 1] = inv_fact[i] * i % MOD

    # Index 0 stands for an empty subtree: size 0, one way.
    size = [0] * (n + 1)
    ways = [1] * (n + 1)
    # Children are inserted after their parent, so reverse insertion order
    # finishes every subtree before its root.
    for v in reversed(nums):
        l, r = left[v], right[v]
        size[v] = size[l] + size[r] + 1
        interleave = fact[size[l] + size[r]] * inv_fact[size[l]] % MOD * inv_fact[size[r]] % MOD
        ways[v] = interleave * ways[l] % MOD * ways[r] % MOD
    return (ways[root] - 1) % MOD
