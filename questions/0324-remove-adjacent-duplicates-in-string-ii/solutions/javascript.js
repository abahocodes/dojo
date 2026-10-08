function removeKDuplicates(s, k) {
  const letters = [];
  const counts = [];
  for (const ch of s) {
    const top = letters.length - 1;
    if (top >= 0 && letters[top] === ch) {
      counts[top] += 1;
      if (counts[top] === k) {
        letters.pop();
        counts.pop();
      }
    } else {
      letters.push(ch);
      counts.push(1);
    }
  }
  const parts = [];
  for (let i = 0; i < letters.length; i++) {
    parts.push(letters[i].repeat(counts[i]));
  }
  return parts.join("");
}
