function editDistance(word1, word2) {
  const m = word2.length;
  let prev = Array.from({ length: m + 1 }, (_, j) => j);
  for (let i = 1; i <= word1.length; i++) {
    const curr = new Array(m + 1).fill(0);
    curr[0] = i;
    for (let j = 1; j <= m; j++) {
      if (word1[i - 1] === word2[j - 1]) {
        curr[j] = prev[j - 1];
      } else {
        curr[j] = 1 + Math.min(prev[j], curr[j - 1], prev[j - 1]);
      }
    }
    prev = curr;
  }
  return prev[m];
}
