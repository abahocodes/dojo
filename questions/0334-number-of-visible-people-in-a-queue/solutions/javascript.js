function canSeePersonsCount(heights) {
  const n = heights.length;
  const answer = new Array(n).fill(0);
  const stack = []; // heights, decreasing from bottom to top
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
