function nextGreaterElement(nums1, nums2) {
  const next = new Map();
  const stack = []; // values still waiting for a greater one, decreasing
  for (const x of nums2) {
    while (stack.length && stack[stack.length - 1] < x) next.set(stack.pop(), x);
    stack.push(x);
  }
  return nums1.map((x) => (next.has(x) ? next.get(x) : -1));
}
