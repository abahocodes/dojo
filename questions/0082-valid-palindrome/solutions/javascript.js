function isPalindrome(s) {
  const keep = (ch) => /[A-Za-z0-9]/.test(ch);
  let left = 0;
  let right = s.length - 1;
  while (left < right) {
    if (!keep(s[left])) {
      left++;
    } else if (!keep(s[right])) {
      right--;
    } else if (s[left].toLowerCase() !== s[right].toLowerCase()) {
      return false;
    } else {
      left++;
      right--;
    }
  }
  return true;
}
