def reverse_parentheses(s):
    n = len(s)
    partner = [0] * n
    opens = []
    for i, ch in enumerate(s):
        if ch == "(":
            opens.append(i)
        elif ch == ")":
            j = opens.pop()
            partner[i], partner[j] = j, i
    out = []
    i, step = 0, 1
    while i < n:
        if s[i] in "()":
            i = partner[i]
            step = -step
        else:
            out.append(s[i])
        i += step
    return "".join(out)
