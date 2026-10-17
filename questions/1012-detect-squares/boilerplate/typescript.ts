// 1012. Detect Squares (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class DetectSquares {
    constructor() {

    }

    add(point: number[]): void {

    }

    count(point: number[]): number {
        return 0;
    }
}

// dojo calls this to run the tests; you don't need to change it.
function runDetectSquares(ops: string[], points: number[][]): number[] {
    const ds = new DetectSquares();
    const out: number[] = [];
    for (let i = 0; i < ops.length; i++) {
        if (ops[i] === "add") ds.add(points[i]);
        else out.push(ds.count(points[i]));
    }
    return out;
}
