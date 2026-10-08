def min_add_to_make_valid(s: str) -> int:
    open_ = added = 0
    for c in s:
        if c == "(":
            open_ += 1
        elif open_:
            open_ -= 1
        else:
            added += 1
    return added + open_
