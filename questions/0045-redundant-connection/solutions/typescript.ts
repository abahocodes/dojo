function findRedundantConnection(edges: number[][]): number[] {
    const n = edges.length;
    const parent = Array.from({ length: n + 1 }, (_, i) => i);
    const size: number[] = new Array(n + 1).fill(1);

    const find = (x: number): number => {
        while (parent[x] !== x) {
            parent[x] = parent[parent[x]]; // path halving
            x = parent[x];
        }
        return x;
    };

    for (const [a, b] of edges) {
        let ra = find(a);
        let rb = find(b);
        // a and b were already connected: this edge closes the cycle.
        if (ra === rb) return [a, b];
        if (size[ra] < size[rb]) [ra, rb] = [rb, ra];
        parent[rb] = ra;
        size[ra] += size[rb];
    }
    return [];
}
