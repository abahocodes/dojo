function reverseVowels(s) {
  const isVowel = (c) => "aeiouAEIOU".includes(c);
  const chars = s.split("");
  let lo = 0;
  let hi = chars.length - 1;
  while (lo < hi) {
    if (!isVowel(chars[lo])) lo++;
    else if (!isVowel(chars[hi])) hi--;
    else {
      [chars[lo], chars[hi]] = [chars[hi], chars[lo]];
      lo++;
      hi--;
    }
  }
  return chars.join("");
}
