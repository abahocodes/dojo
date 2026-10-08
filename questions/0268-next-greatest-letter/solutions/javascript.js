function nextGreatestLetter(letters, target) {
  let lo = 0;
  let hi = letters.length;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (letters[mid] <= target) lo = mid + 1;
    else hi = mid;
  }
  return letters[lo % letters.length];
}
