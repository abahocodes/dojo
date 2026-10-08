function stockSpan(prices) {
  const n = prices.length;
  const result = new Array(n);
  const stack = [];
  for (let i = 0; i < n; i++) {
    while (stack.length > 0 && prices[stack[stack.length - 1]] <= prices[i]) {
      stack.pop();
    }
    result[i] = stack.length > 0 ? i - stack[stack.length - 1] : i + 1;
    stack.push(i);
  }
  return result;
}
