function containerWithMostWater(heights: number[]): number {
  let left = 0;
  let right = heights.length - 1;
  let best = 0;
  while (left < right) {
    const width = right - left;
    if (heights[left] < heights[right]) {
      best = Math.max(best, width * heights[left]);
      left++;
    } else {
      best = Math.max(best, width * heights[right]);
      right--;
    }
  }
  return best;
}
