def min_subarray_remove(nums: list[int], p: int) -> int:
    need = sum(nums) % p
    if need == 0:
        return 0
    latest = {0: -1}
    cur = 0
    best = len(nums)
    for j, x in enumerate(nums):
        cur = (cur + x) % p
        want = (cur - need) % p
        if want in latest:
            best = min(best, j - latest[want])
        latest[cur] = j
    return best if best < len(nums) else -1
