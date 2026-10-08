function largestNumber(nums: number[]): string {
  const parts = nums.map(String);
  parts.sort((a, b) => {
    const ab = a + b, ba = b + a;
    return ab === ba ? 0 : ab > ba ? -1 : 1;
  });
  const result = parts.join("");
  return result[0] === "0" ? "0" : result;
}
