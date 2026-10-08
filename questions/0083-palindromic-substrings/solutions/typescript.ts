function countSubstrings(s: string): number {
  const n = s.length;
  let count = 0;
  // Every palindrome has a center: a character (odd length) or a gap (even length).
  for (let center = 0; center < 2 * n - 1; center++) {
    let left = Math.floor(center / 2);
    let right = left + (center % 2);
    while (left >= 0 && right < n && s[left] === s[right]) {
      count++;
      left--;
      right++;
    }
  }
  return count;
}
