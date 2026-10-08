def is_long_pressed_name(name: str, typed: str) -> bool:
    i = 0
    for j, c in enumerate(typed):
        if i < len(name) and name[i] == c:
            i += 1
        elif j == 0 or typed[j - 1] != c:
            return False
    return i == len(name)
