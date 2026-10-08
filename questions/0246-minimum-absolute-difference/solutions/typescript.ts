function minimumAbsDifference(arr: number[]): number[][] {
  const a = [...arr].sort((x, y) => x - y);
  let best = Infinity;
  for (let i = 0; i + 1 < a.length; i++) best = Math.min(best, a[i + 1] - a[i]);
  const out: number[][] = [];
  for (let i = 0; i + 1 < a.length; i++) {
    if (a[i + 1] - a[i] === best) out.push([a[i], a[i + 1]]);
  }
  return out;
}
