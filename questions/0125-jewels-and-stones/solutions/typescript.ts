function numJewelsInStones(jewels: string, stones: string): number {
  const kinds = new Set<string>(jewels);
  let count = 0;
  for (const c of stones) if (kinds.has(c)) count++;
  return count;
}
