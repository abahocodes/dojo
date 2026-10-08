def count_of_atoms(formula: str) -> str:
    n = len(formula)
    stack = [{}]  # one count map per open group; bottom = whole formula
    i = 0

    def read_number() -> int:
        nonlocal i
        start = i
        while i < n and formula[i].isdigit():
            i += 1
        return int(formula[start:i]) if i > start else 1

    while i < n:
        ch = formula[i]
        if ch == "(":
            stack.append({})
            i += 1
        elif ch == ")":
            i += 1
            mult = read_number()
            group = stack.pop()
            top = stack[-1]
            for name, cnt in group.items():
                top[name] = top.get(name, 0) + cnt * mult
        else:
            start = i
            i += 1
            while i < n and formula[i].islower():
                i += 1
            name = formula[start:i]
            stack[-1][name] = stack[-1].get(name, 0) + read_number()

    counts = stack[0]
    return "".join(name + (str(counts[name]) if counts[name] > 1 else "")
                   for name in sorted(counts))
