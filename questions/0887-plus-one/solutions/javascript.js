function plusOne(digits) {
  // Walk from the last digit: a 9 becomes 0 and carries; anything else
  // just increments and we're done.
  const result = digits.slice();
  for (let i = result.length - 1; i >= 0; i--) {
    if (result[i] < 9) {
      result[i]++;
      return result;
    }
    result[i] = 0;
  }
  // Every digit was 9: the answer is 1 followed by zeros.
  result.unshift(1);
  return result;
}
