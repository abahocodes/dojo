function countPrefixes(words, s) {
  let count = 0;
  for (const word of words) {
    if (s.startsWith(word)) count++;
  }
  return count;
}
