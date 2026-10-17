def my_pow(x: float, n: int) -> float:
    # Binary exponentiation: square the base and halve the exponent,
    # multiplying the base into the result whenever the low bit is set.
    if n < 0:
        x = 1 / x
        n = -n
    result = 1.0
    while n > 0:
        if n & 1:
            result *= x
        x *= x
        n >>= 1
    return result
