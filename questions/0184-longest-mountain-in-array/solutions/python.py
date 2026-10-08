def longest_mountain(arr: list[int]) -> int:
    best = up = down = 0
    for i in range(1, len(arr)):
        if arr[i - 1] == arr[i] or (down > 0 and arr[i - 1] < arr[i]):
            up = down = 0
        if arr[i - 1] < arr[i]:
            up += 1
        elif arr[i - 1] > arr[i]:
            down += 1
        if up > 0 and down > 0:
            best = max(best, up + down + 1)
    return best
