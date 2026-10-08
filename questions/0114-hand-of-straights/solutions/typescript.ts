function isNStraightHand(hand: number[], groupSize: number): boolean {
  if (hand.length % groupSize !== 0) return false;
  const count = new Map<number, number>();
  for (const x of hand) count.set(x, (count.get(x) ?? 0) + 1);
  const keys = [...count.keys()].sort((a, b) => a - b);
  for (const x of keys) {
    const c = count.get(x)!;
    if (c === 0) continue;
    for (let v = x; v < x + groupSize; v++) {
      const have = count.get(v) ?? 0;
      if (have < c) return false;
      count.set(v, have - c);
    }
  }
  return true;
}
