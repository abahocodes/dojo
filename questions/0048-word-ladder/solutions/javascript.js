function ladderLength(beginWord, endWord, wordList) {
  const words = new Set(wordList);
  if (!words.has(endWord)) return 0;
  const letters = "abcdefghijklmnopqrstuvwxyz";
  words.delete(beginWord);

  // Array used as a FIFO queue with a moving head index.
  const queue = [[beginWord, 1]];
  let head = 0;
  while (head < queue.length) {
    const [word, steps] = queue[head++];
    for (let i = 0; i < word.length; i++) {
      const prefix = word.slice(0, i);
      const suffix = word.slice(i + 1);
      for (const ch of letters) {
        const candidate = prefix + ch + suffix;
        if (words.has(candidate)) {
          if (candidate === endWord) return steps + 1;
          words.delete(candidate); // visited: never enqueue twice
          queue.push([candidate, steps + 1]);
        }
      }
    }
  }
  return 0;
}
