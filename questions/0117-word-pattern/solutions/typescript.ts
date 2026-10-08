function wordPattern(pattern: string, s: string): boolean {
  const words = s.split(' ');
  if (words.length !== pattern.length) return false;
  const letterToWord = new Map<string, string>();
  const wordToLetter = new Map<string, string>();
  for (let i = 0; i < words.length; i++) {
    const c = pattern[i];
    const w = words[i];
    if (letterToWord.has(c) ? letterToWord.get(c) !== w : wordToLetter.has(w)) return false;
    letterToWord.set(c, w);
    wordToLetter.set(w, c);
  }
  return true;
}
