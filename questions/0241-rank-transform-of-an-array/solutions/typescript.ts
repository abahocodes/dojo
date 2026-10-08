function arrayRankTransform(arr: number[]): number[] {
  const sorted = [...new Set(arr)].sort((a, b) => a - b);
  const rank = new Map<number, number>();
  sorted.forEach((x, i) => rank.set(x, i + 1));
  return arr.map((x) => rank.get(x)!);
}
