// 1012. Detect Squares (medium)
// /show to reread the problem · /test runs visible tests · /submit when done
package main

type DetectSquares struct {
}

func Constructor() DetectSquares {
	return DetectSquares{}
}

func (d *DetectSquares) Add(point []int) {

}

func (d *DetectSquares) Count(point []int) int {
	return 0
}

// dojo calls this to run the tests; you don't need to change it.
func runDetectSquares(ops []string, points [][]int) []int {
	ds := Constructor()
	out := []int{}
	for i, op := range ops {
		if op == "add" {
			ds.Add(points[i])
		} else {
			out = append(out, ds.Count(points[i]))
		}
	}
	return out
}
