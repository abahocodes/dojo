def calculate_no_parens(s: str) -> int:
    total = last = num = 0
    op = "+"
    n = len(s)
    for i, ch in enumerate(s):
        if ch.isdigit():
            num = num * 10 + ord(ch) - 48
        if (ch != " " and not ch.isdigit()) or i == n - 1:
            if op == "+":
                total, last = total + last, num
            elif op == "-":
                total, last = total + last, -num
            elif op == "*":
                last *= num
            else:
                q = abs(last) // num
                last = q if last >= 0 else -q
            op, num = ch, 0
    return total + last
