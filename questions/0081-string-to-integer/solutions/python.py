def string_to_integer(s: str) -> int:
    INT_MAX = 2**31 - 1
    INT_MIN = -(2**31)
    i, n = 0, len(s)
    while i < n and s[i] == " ":
        i += 1
    sign = 1
    if i < n and s[i] in "+-":
        if s[i] == "-":
            sign = -1
        i += 1
    value = 0
    while i < n and "0" <= s[i] <= "9":
        value = value * 10 + (ord(s[i]) - ord("0"))
        if value > INT_MAX:  # stop early: the result is clamped anyway
            return INT_MAX if sign == 1 else INT_MIN
        i += 1
    return sign * value
