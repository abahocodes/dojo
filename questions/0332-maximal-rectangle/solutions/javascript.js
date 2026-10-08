function maximalRectangle(matrix) {
  const cols = matrix[0].length;
  const heights = new Array(cols + 1).fill(0); // heights[cols] stays 0
  const stack = new Array(cols + 1);
  let best = 0;
  for (const row of matrix) {
    for (let c = 0; c < cols; c++) {
      heights[c] = row[c] === "1" ? heights[c] + 1 : 0;
    }
    let top = 0;
    for (let i = 0; i <= cols; i++) {
      while (top > 0 && heights[stack[top - 1]] >= heights[i]) {
        const h = heights[stack[--top]];
        const left = top > 0 ? stack[top - 1] : -1;
        best = Math.max(best, h * (i - left - 1));
      }
      stack[top++] = i;
    }
  }
  return best;
}
