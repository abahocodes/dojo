function stockSpan(prices: number[]): number[] {
  const n = prices.length;
  const result: number[] = new Array(n);
  const stack: number[] = [];
  for (let i = 0; i < n; i++) {
    while (stack.length > 0 && prices[stack[stack.length - 1]] <= prices[i]) {
      stack.pop();
    }
    result[i] = stack.length > 0 ? i - stack[stack.length - 1] : i + 1;
    stack.push(i);
  }
  return result;
}
