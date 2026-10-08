function closeStrings(word1, word2) {
  if (word1.length !== word2.length) return false;
  const a = new Array(26).fill(0);
  const b = new Array(26).fill(0);
  for (let i = 0; i < word1.length; i++) a[word1.charCodeAt(i) - 97]++;
  for (let i = 0; i < word2.length; i++) b[word2.charCodeAt(i) - 97]++;
  for (let i = 0; i < 26; i++) {
    if ((a[i] === 0) !== (b[i] === 0)) return false;
  }
  a.sort((x, y) => x - y);
  b.sort((x, y) => x - y);
  return a.every((x, i) => x === b[i]);
}
