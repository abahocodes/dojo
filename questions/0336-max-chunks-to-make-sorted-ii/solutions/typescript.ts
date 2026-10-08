function maxChunksToSorted(arr: number[]): number {
  const stack: number[] = []; // maximum of each chunk, non-decreasing
  for (const x of arr) {
    if (stack.length === 0 || x >= stack[stack.length - 1]) {
      stack.push(x);
    } else {
      const biggest = stack[stack.length - 1];
      while (stack.length > 0 && stack[stack.length - 1] > x) stack.pop();
      stack.push(biggest);
    }
  }
  return stack.length;
}
