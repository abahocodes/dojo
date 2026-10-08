function customSortString(order, s) {
  const counts = new Array(26).fill(0);
  for (const c of s) counts[c.charCodeAt(0) - 97]++;
  const ranked = new Set(order);
  let out = "";
  for (const c of order) out += c.repeat(counts[c.charCodeAt(0) - 97]);
  for (const c of s) if (!ranked.has(c)) out += c;
  return out;
}
