def decrypt(code: list[int], k: int) -> list[int]:
    n = len(code)
    result = [0] * n
    if k == 0:
        return result
    start, end = (1, k) if k > 0 else (n + k, n - 1)
    window = sum(code[j % n] for j in range(start, end + 1))
    for i in range(n):
        result[i] = window
        window -= code[start % n]
        start += 1
        end += 1
        window += code[end % n]
    return result
