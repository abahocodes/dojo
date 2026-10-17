def multiply(num1: str, num2: str) -> str:
    # Schoolbook multiplication: digit i of num1 times digit j of num2 lands
    # in positions i + j and i + j + 1 of the product (counting from the left).
    if num1 == "0" or num2 == "0":
        return "0"
    m, n = len(num1), len(num2)
    out = [0] * (m + n)
    for i in range(m - 1, -1, -1):
        a = ord(num1[i]) - 48
        for j in range(n - 1, -1, -1):
            total = a * (ord(num2[j]) - 48) + out[i + j + 1]
            out[i + j + 1] = total % 10
            out[i + j] += total // 10
    start = 0
    while start < len(out) - 1 and out[start] == 0:
        start += 1
    return "".join(str(d) for d in out[start:])
