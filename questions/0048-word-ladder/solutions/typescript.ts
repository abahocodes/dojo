function ladderLength(beginWord: string, endWord: string, wordList: string[]): number {
  const words = new Set(wordList);
  if (!words.has(endWord)) return 0;
  const letters = "abcdefghijklmnopqrstuvwxyz";
  words.delete(beginWord);
  const queue: [string, number][] = [[beginWord, 1]];
  for (let head = 0; head < queue.length; head++) {
    const [word, steps] = queue[head];
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
