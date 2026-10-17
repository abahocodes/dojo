// 1002. Min Stack (medium)
// /show to reread the problem · /test runs visible tests · /submit when done
package main

type MinStack struct {
}

func Constructor() MinStack {
	return MinStack{}
}

func (this *MinStack) Push(val int) {

}

func (this *MinStack) Pop() {

}

func (this *MinStack) Top() int {
	return 0
}

func (this *MinStack) GetMin() int {
	return 0
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
