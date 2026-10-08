function eraseOverlapIntervals(intervals: number[][]): number {
    let removed = 0;
    let lastEnd = -Infinity;
    // keeping the interval that ends first leaves the most room for the rest
    const byEnd = [...intervals].sort((a, b) => a[1] - b[1]);
    for (const [start, end] of byEnd) {
        if (start >= lastEnd) lastEnd = end;
        else removed++;
    }
    return removed;
}
