MOD = 10**9 + 7


def three_sum_multi(arr: list[int], target: int) -> int:
    c = [0] * 101
    for v in arr:
        c[v] += 1
    total = 0
    for x in range(101):
        for y in range(x, 101):
            z = target - x - y
            if z < y or z > 100:
                continue
            if x == y == z:
                total += c[x] * (c[x] - 1) * (c[x] - 2) // 6
            elif x == y:
                total += c[x] * (c[x] - 1) // 2 * c[z]
            elif y == z:
                total += c[x] * c[y] * (c[y] - 1) // 2
            else:
                total += c[x] * c[y] * c[z]
    return total % MOD
