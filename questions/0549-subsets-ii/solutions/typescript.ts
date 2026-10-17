function subsetsWithDup(nums: number[]): number[][] {
    const sorted = [...nums].sort((a, b) => a - b); // equal values become neighbours
    const result: number[][] = [];
    const current: number[] = [];

    const backtrack = (start: number): void => {
        result.push([...current]);
        for (let i = start; i < sorted.length; i++) {
            if (i > start && sorted[i] === sorted[i - 1]) continue; // already tried here
            current.push(sorted[i]);
            backtrack(i + 1);
            current.pop();
        }
    };

    backtrack(0);
    return result;
}
