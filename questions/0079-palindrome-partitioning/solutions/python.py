def partition(s: str) -> list[list[str]]:
    n = len(s)
    # pal[i][j] is True when s[i..j] is a palindrome
    pal = [[False] * n for _ in range(n)]
    for i in range(n - 1, -1, -1):
        for j in range(i, n):
            if s[i] == s[j] and (j - i < 2 or pal[i + 1][j - 1]):
                pal[i][j] = True

    result = []
    current = []

    def backtrack(start: int) -> None:
        if start == n:
            result.append(current[:])
            return
        for end in range(start, n):
            if pal[start][end]:
                current.append(s[start:end + 1])
                backtrack(end + 1)
                current.pop()

    backtrack(0)
    return result
