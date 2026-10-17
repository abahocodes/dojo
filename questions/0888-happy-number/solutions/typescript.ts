function isHappy(n: number): boolean {
  // The digit-square sequence either reaches 1 or falls into a cycle.
  // Floyd's tortoise and hare finds out without remembering every value.
  const step = (v: number): number => {
    let total = 0;
    while (v > 0) {
      const d = v % 10;
      total += d * d;
      v = Math.floor(v / 10);
    }
    return total;
  };
  let slow = n;
  let fast = step(n);
  while (fast !== 1 && slow !== fast) {
    slow = step(slow);
    fast = step(step(fast));
  }
  return fast === 1;
}
