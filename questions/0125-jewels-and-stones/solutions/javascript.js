function numJewelsInStones(jewels, stones) {
  const kinds = new Set(jewels);
  let count = 0;
  for (const c of stones) if (kinds.has(c)) count++;
  return count;
}
