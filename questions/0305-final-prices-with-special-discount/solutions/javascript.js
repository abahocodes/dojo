function finalPrices(prices) {
  const result = prices.slice();
  const stack = []; // indices still waiting for a discount; their prices increase
  for (let j = 0; j < prices.length; j++) {
    while (stack.length && prices[stack[stack.length - 1]] >= prices[j]) {
      result[stack.pop()] -= prices[j];
    }
    stack.push(j);
  }
  return result;
}
