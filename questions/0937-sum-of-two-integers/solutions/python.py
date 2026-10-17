def get_sum(a, b):
    MASK = 0xFFFFFFFF
    MAX = 0x7FFFFFFF
    a &= MASK
    b &= MASK
    while b:
        carry = ((a & b) << 1) & MASK
        a = a ^ b
        b = carry
    return a if a <= MAX else ~(a ^ MASK)
