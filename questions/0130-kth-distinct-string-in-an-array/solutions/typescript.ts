function kthDistinct(arr: string[], k: number): string {
  const count = new Map<string, number>();
  for (const s of arr) count.set(s, (count.get(s) ?? 0) + 1);
  for (const s of arr) {
    if (count.get(s) === 1 && --k === 0) return s;
  }
  return "";
}
