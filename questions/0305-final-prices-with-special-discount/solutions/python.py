def final_prices(prices: list[int]) -> list[int]:
    result = prices[:]
    stack = []  # indices still waiting for a discount; their prices increase
    for j, p in enumerate(prices):
        while stack and prices[stack[-1]] >= p:
            result[stack.pop()] -= p
        stack.append(j)
    return result
