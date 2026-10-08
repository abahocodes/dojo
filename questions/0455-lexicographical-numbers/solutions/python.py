def lexical_order(n: int) -> list[int]:
    result = []
    cur = 1
    for _ in range(n):
        result.append(cur)
        if cur * 10 <= n:
            cur *= 10
        else:
            while cur % 10 == 9 or cur + 1 > n:
                cur //= 10
            cur += 1
    return result
