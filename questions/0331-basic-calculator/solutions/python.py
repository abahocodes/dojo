def calculate(s):
    result, num, sign = 0, 0, 1
    stack = []
    for ch in s:
        if ch.isdigit():
            num = num * 10 + (ord(ch) - 48)
        elif ch == "+" or ch == "-":
            result += sign * num
            num = 0
            sign = 1 if ch == "+" else -1
        elif ch == "(":
            stack.append(result)
            stack.append(sign)
            result, sign = 0, 1
        elif ch == ")":
            result += sign * num
            num = 0
            saved_sign = stack.pop()
            saved_result = stack.pop()
            result = saved_result + saved_sign * result
    return result + sign * num
