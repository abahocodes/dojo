def relative_sort_array(arr1: list[int], arr2: list[int]) -> list[int]:
    count = [0] * 1001
    for x in arr1:
        count[x] += 1
    result = []
    for x in arr2:
        result.extend([x] * count[x])
        count[x] = 0
    for x in range(1001):
        result.extend([x] * count[x])
    return result
