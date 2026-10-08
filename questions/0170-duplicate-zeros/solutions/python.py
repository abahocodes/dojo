def duplicate_zeros(arr: list[int]) -> list[int]:
    out = list(arr)
    n = len(out)
    # shift = number of zeros strictly before index i: out[i] lands at i + shift.
    shift = out.count(0)
    for i in range(n - 1, -1, -1):
        if out[i] == 0:
            shift -= 1
            if i + shift + 1 < n:
                out[i + shift + 1] = 0
        if i + shift < n:
            out[i + shift] = out[i]
    return out
