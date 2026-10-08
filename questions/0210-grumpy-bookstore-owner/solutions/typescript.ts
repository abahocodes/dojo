function maxSatisfied(customers: number[], grumpy: number[], minutes: number): number {
  let base = 0;
  let gain = 0;
  let best = 0;
  for (let i = 0; i < customers.length; i++) {
    if (grumpy[i] === 0) base += customers[i];
    else gain += customers[i];
    if (i >= minutes && grumpy[i - minutes] === 1) gain -= customers[i - minutes];
    best = Math.max(best, gain);
  }
  return base + best;
}
