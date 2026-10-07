function trap(height) {
  let lo = 0;
  let hi = height.length - 1;
  let leftMax = 0;
  let rightMax = 0;
  let water = 0;
  while (lo <= hi) {
    if (leftMax <= rightMax) {
      leftMax = Math.max(leftMax, height[lo]);
      water += leftMax - height[lo++];
    } else {
      rightMax = Math.max(rightMax, height[hi]);
      water += rightMax - height[hi--];
    }
  }
  return water;
}
