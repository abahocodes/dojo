def min_k_bit_flips(nums: list[int], k: int) -> int:
    n = len(nums)
    ends = [0] * (n + 1)  # ends[j] = 1 if a flip started at j - k stops covering at j
    active = 0            # parity of flips currently covering index i
    flips = 0
    for i in range(n):
        active ^= ends[i]
        if nums[i] ^ active == 0:  # this bit is still 0
            if i + k > n:
                return -1
            flips += 1
            active ^= 1
            ends[i + k] ^= 1
    return flips
