function thirdMax(nums) {
  let first = null, second = null, third = null;
  for (const x of nums) {
    if (x === first || x === second || x === third) continue;
    if (first === null || x > first) {
      third = second; second = first; first = x;
    } else if (second === null || x > second) {
      third = second; second = x;
    } else if (third === null || x > third) {
      third = x;
    }
  }
  return third === null ? first : third;
}
