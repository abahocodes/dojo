def generate_parenthesis(n: int) -> list[str]:
    result, current = [], []

    def backtrack(open_, close):
        if len(current) == 2 * n:
            result.append("".join(current))
            return
        if open_ < n:
            current.append("(")
            backtrack(open_ + 1, close)
            current.pop()
        if close < open_:
            current.append(")")
            backtrack(open_, close + 1)
            current.pop()

    backtrack(0, 0)
    return result
