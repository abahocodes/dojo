function characterReplacement(s, k) {
  const counts = new Array(26).fill(0);
  let maxFreq = 0;
  let left = 0;
  for (let right = 0; right < s.length; right++) {
    const idx = s.charCodeAt(right) - 65;
    counts[idx]++;
    if (counts[idx] > maxFreq) maxFreq = counts[idx];
    if (right - left + 1 - maxFreq > k) {
      counts[s.charCodeAt(left) - 65]--;
      left++;
    }
  }
  return s.length - left;
}
