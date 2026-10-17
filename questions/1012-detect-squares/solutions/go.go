package main

type DetectSquares struct {
	cnt map[[2]int]int // (x, y) -> copies stored
	ys  map[int][]int  // x -> distinct ys stored in that column
}

func Constructor() DetectSquares {
	return DetectSquares{cnt: map[[2]int]int{}, ys: map[int][]int{}}
}

func (d *DetectSquares) Add(point []int) {
	key := [2]int{point[0], point[1]}
	if d.cnt[key] == 0 {
		d.ys[point[0]] = append(d.ys[point[0]], point[1])
	}
	d.cnt[key]++
}

func (d *DetectSquares) Count(point []int) int {
	qx, qy := point[0], point[1]
	total := 0
	// Every point in the query's column fixes the side length.
	for _, y := range d.ys[qx] {
		side := y - qy
		if side == 0 {
			continue
		}
		c := d.cnt[[2]int{qx, y}]
		for _, px := range []int{qx + side, qx - side} {
			total += c * d.cnt[[2]int{px, qy}] * d.cnt[[2]int{px, y}]
		}
	}
	return total
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
