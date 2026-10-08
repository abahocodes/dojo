function minimumLengthEncoding(words) {
  const keep = new Set(words);
  for (const w of new Set(words)) {
    for (let k = 1; k < w.length; k++) keep.delete(w.slice(k));
  }
  let total = 0;
  for (const w of keep) total += w.length + 1;
  return total;
}
