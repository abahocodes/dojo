package main

type MinStack struct {
	// values[i] is an entry; mins[i] is the minimum of values[0..i].
	values []int
	mins   []int
}

func Constructor() MinStack {
	return MinStack{}
}

func (this *MinStack) Push(val int) {
	low := val
	if n := len(this.mins); n > 0 && this.mins[n-1] < low {
		low = this.mins[n-1]
	}
	this.values = append(this.values, val)
	this.mins = append(this.mins, low)
}

func (this *MinStack) Pop() {
	this.values = this.values[:len(this.values)-1]
	this.mins = this.mins[:len(this.mins)-1]
}

func (this *MinStack) Top() int {
	return this.values[len(this.values)-1]
}

func (this *MinStack) GetMin() int {
	return this.mins[len(this.mins)-1]
}

// dojo calls this to run the tests; you don't need to change it.
func minStack(ops []string, vals []int) []int {
	stack := Constructor()
	out := []int{}
	for i, op := range ops {
		switch op {
		case "push":
			stack.Push(vals[i])
		case "pop":
			stack.Pop()
		case "top":
			out = append(out, stack.Top())
		default:
			out = append(out, stack.GetMin())
		}
	}
	return out
}
