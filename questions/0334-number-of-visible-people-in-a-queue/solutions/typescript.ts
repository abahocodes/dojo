function canSeePersonsCount(heights: number[]): number[] {
  const n = heights.length;
  const answer: number[] = new Array(n).fill(0);
  const stack: number[] = []; // heights, decreasing from bottom to top
  for (let i = n - 1; i >= 0; i--) {
    let seen = 0;
    while (stack.length > 0 && stack[stack.length - 1] < heights[i]) {
      stack.pop();
      seen++;
    }
    if (stack.length > 0) seen++; // the first taller person
    answer[i] = seen;
    stack.push(heights[i]);
  }
  return answer;
}
