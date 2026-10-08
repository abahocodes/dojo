function prefixCount(words: string[], pref: string): number {
  let count = 0;
  for (const word of words) {
    if (word.startsWith(pref)) count++;
  }
  return count;
}
