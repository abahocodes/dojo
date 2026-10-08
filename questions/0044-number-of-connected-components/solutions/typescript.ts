function countComponents(n: number, edges: number[][]): number {
    const parent = Array.from({ length: n }, (_, i) => i);
    const size: number[] = new Array(n).fill(1);

    const find = (x: number): number => {
        while (parent[x] !== x) {
            parent[x] = parent[parent[x]]; // path halving
            x = parent[x];
        }
        return x;
    };

    let components = n;
    for (const [a, b] of edges) {
        let ra = find(a);
        let rb = find(b);
        if (ra === rb) continue;
        if (size[ra] < size[rb]) [ra, rb] = [rb, ra];
        parent[rb] = ra;
        size[ra] += size[rb];
        components--;
    }
    return components;
}
