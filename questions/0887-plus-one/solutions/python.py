def plus_one(digits: list[int]) -> list[int]:
    # Walk from the last digit: a 9 becomes 0 and carries; anything else
    # just increments and we're done.
    result = list(digits)
    for i in range(len(result) - 1, -1, -1):
        if result[i] < 9:
            result[i] += 1
            return result
        result[i] = 0
    # Every digit was 9: the answer is 1 followed by zeros.
    return [1] + result
