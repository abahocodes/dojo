function findLucky(arr: number[]): number {
  const count: number[] = new Array(501).fill(0);
  for (const x of arr) count[x]++;
  for (let v = 500; v >= 1; v--) {
    if (count[v] === v) return v;
  }
  return -1;
}
