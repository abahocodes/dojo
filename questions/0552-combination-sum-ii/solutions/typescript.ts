function combinationSum2(candidates: number[], target: number): number[][] {
    const sorted = [...candidates].sort((a, b) => a - b); // equal values become neighbours
    const result: number[][] = [];
    const current: number[] = [];

    const backtrack = (start: number, remaining: number): void => {
        if (remaining === 0) {
            result.push([...current]);
            return;
        }
        for (let i = start; i < sorted.length; i++) {
            const c = sorted[i];
            if (c > remaining) break; // every later candidate is too big as well
            if (i > start && c === sorted[i - 1]) continue; // already tried here
            current.push(c);
            backtrack(i + 1, remaining - c); // i + 1: each card is used once
            current.pop();
        }
    };

    backtrack(0, target);
    return result;
}
