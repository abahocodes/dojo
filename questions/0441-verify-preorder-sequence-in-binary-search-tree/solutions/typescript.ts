function verifyPreorder(preorder: number[]): boolean {
  let low = -Infinity;
  const stack: number[] = [];
  for (const x of preorder) {
    if (x < low) return false;
    while (stack.length > 0 && stack[stack.length - 1] < x) {
      low = stack.pop()!;
    }
    stack.push(x);
  }
  return true;
}
