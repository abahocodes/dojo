function minEatingSpeed(piles: number[], h: number): number {
    let lo = 1;
    let hi = Math.max(...piles);
    while (lo < hi) {
        const v = Math.floor((lo + hi) / 2);
        let hours = 0;
        for (const p of piles) {
            hours += Math.ceil(p / v);
        }
        if (hours <= h) {
            hi = v;
        } else {
            lo = v + 1;
        }
    }
    return lo;
}
