function totalFruit(fruits: number[]): number {
  const count = new Map<number, number>();
  let left = 0;
  let best = 0;
  for (let right = 0; right < fruits.length; right++) {
    const f = fruits[right];
    count.set(f, (count.get(f) ?? 0) + 1);
    while (count.size > 2) {
      const g = fruits[left++];
      const c = count.get(g)! - 1;
      if (c === 0) count.delete(g);
      else count.set(g, c);
    }
    best = Math.max(best, right - left + 1);
  }
  return best;
}
