function countPrefixSuffixPairs(words) {
  let count = 0;
  for (let j = 0; j < words.length; j++) {
    for (let i = 0; i < j; i++) {
      if (words[j].startsWith(words[i]) && words[j].endsWith(words[i])) count++;
    }
  }
  return count;
}
