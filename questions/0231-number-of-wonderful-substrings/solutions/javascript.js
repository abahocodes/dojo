function wonderfulSubstrings(word) {
  const seen = new Array(1024).fill(0);
  seen[0] = 1;
  let mask = 0;
  let total = 0;
  for (let i = 0; i < word.length; i++) {
    mask ^= 1 << (word.charCodeAt(i) - 97);
    total += seen[mask];
    for (let k = 0; k < 10; k++) total += seen[mask ^ (1 << k)];
    seen[mask]++;
  }
  return total;
}
