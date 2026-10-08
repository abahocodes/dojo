function nextGreaterElement(nums1: number[], nums2: number[]): number[] {
  const next = new Map<number, number>();
  const stack: number[] = []; // values still waiting for a greater one, decreasing
  for (const x of nums2) {
    while (stack.length && stack[stack.length - 1] < x) next.set(stack.pop()!, x);
    stack.push(x);
  }
  return nums1.map((x) => next.get(x) ?? -1);
}
