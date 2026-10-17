def reverse_integer(x):
    INT_MAX = 2**31 - 1
    sign = -1 if x < 0 else 1
    x = abs(x)
    limit = INT_MAX if sign == 1 else INT_MAX + 1
    rev = 0
    while x:
        digit = x % 10
        x //= 10
        if rev > limit // 10 or (rev == limit // 10 and digit > limit % 10):
            return 0
        rev = rev * 10 + digit
    return sign * rev
