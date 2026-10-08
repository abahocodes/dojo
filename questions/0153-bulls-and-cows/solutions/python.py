def get_hint(secret: str, guess: str) -> str:
    bulls = cows = 0
    bal = [0] * 10
    for s, g in zip(secret, guess):
        if s == g:
            bulls += 1
            continue
        a, b = ord(s) - 48, ord(g) - 48
        if bal[a] < 0:
            cows += 1
        if bal[b] > 0:
            cows += 1
        bal[a] += 1
        bal[b] -= 1
    return f"{bulls}A{cows}B"
