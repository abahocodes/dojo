function twoSumSorted(numbers: number[], target: number): number[] {
  let lo = 0;
  let hi = numbers.length - 1;
  while (lo < hi) {
    const total = numbers[lo] + numbers[hi];
    if (total === target) return [lo + 1, hi + 1];
    if (total < target) lo++;
    else hi--;
  }
  return [];
}
