def find_length_of_shortest_subarray(arr: list[int]) -> int:
    n = len(arr)
    right = n - 1
    while right > 0 and arr[right - 1] <= arr[right]:
        right -= 1
    if right == 0:
        return 0
    best = right
    for left in range(n):
        if left > 0 and arr[left - 1] > arr[left]:
            break
        while right < n and arr[right] < arr[left]:
            right += 1
        best = min(best, right - left - 1)
    return best
