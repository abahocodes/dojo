function minSteps(s: string, t: string): number {
  const diff: number[] = new Array(26).fill(0);
  for (let i = 0; i < s.length; i++) diff[s.charCodeAt(i) - 97]++;
  for (let i = 0; i < t.length; i++) diff[t.charCodeAt(i) - 97]--;
  let steps = 0;
  for (const d of diff) if (d > 0) steps += d;
  return steps;
}
