function largestRectangleArea(heights: number[]): number {
    const stack: number[] = [];
    let best = 0;
    const n = heights.length;
    for (let i = 0; i <= n; i++) {
        const h = i < n ? heights[i] : 0;
        while (stack.length > 0 && heights[stack[stack.length - 1]] >= h) {
            const height = heights[stack.pop()!];
            const left = stack.length > 0 ? stack[stack.length - 1] : -1;
            best = Math.max(best, height * (i - left - 1));
        }
        stack.push(i);
    }
    return best;
}
