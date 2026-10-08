function mergeAlternately(word1, word2) {
  const out = [];
  for (let i = 0; i < word1.length || i < word2.length; i++) {
    if (i < word1.length) out.push(word1[i]);
    if (i < word2.length) out.push(word2[i]);
  }
  return out.join("");
}
