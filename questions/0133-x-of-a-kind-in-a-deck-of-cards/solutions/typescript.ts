function hasGroupsSizeX(deck: number[]): boolean {
  const gcd = (a: number, b: number): number => {
    while (b) [a, b] = [b, a % b];
    return a;
  };
  const counts = new Map<number, number>();
  for (const v of deck) counts.set(v, (counts.get(v) ?? 0) + 1);
  let g = 0;
  for (const c of counts.values()) g = gcd(g, c);
  return g >= 2;
}
