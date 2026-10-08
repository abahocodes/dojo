function commonChars(words) {
  let common = new Array(26).fill(Infinity);
  for (const w of words) {
    const freq = new Array(26).fill(0);
    for (let i = 0; i < w.length; i++) freq[w.charCodeAt(i) - 97]++;
    for (let i = 0; i < 26; i++) common[i] = Math.min(common[i], freq[i]);
  }
  const result = [];
  for (let i = 0; i < 26; i++) {
    for (let j = 0; j < common[i]; j++) result.push(String.fromCharCode(97 + i));
  }
  return result;
}
