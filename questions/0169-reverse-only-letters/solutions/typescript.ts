function reverseOnlyLetters(s: string): string {
  const isLetter = (c: string): boolean => (c >= "a" && c <= "z") || (c >= "A" && c <= "Z");
  const chars = s.split("");
  let lo = 0;
  let hi = chars.length - 1;
  while (lo < hi) {
    if (!isLetter(chars[lo])) lo++;
    else if (!isLetter(chars[hi])) hi--;
    else {
      [chars[lo], chars[hi]] = [chars[hi], chars[lo]];
      lo++;
      hi--;
    }
  }
  return chars.join("");
}
