// dojo's Go driver support, compiled with every Go solution. Same protocol
// as harness.py: read the spec, run each case, rewrite the results file after
// every case and name the current step in the progress file. Comparison
// happens in dojo.
package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"reflect"
	"runtime/debug"
	"strings"
	"time"
)

// ListNode and TreeNode as on LeetCode.
type ListNode struct {
	Val  int
	Next *ListNode
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

const dojoStdoutCap = 4000
const dojoMaxNodes = 100000

type dojoCase struct {
	Index int                        `json:"index"`
	Input map[string]json.RawMessage `json:"input"`
}

type dojoSpec struct {
	Cases []dojoCase `json:"cases"`
}

type dojoResult struct {
	Index  int     `json:"index"`
	Status string  `json:"status"`
	Got    any     `json:"got,omitempty"`
	Error  string  `json:"error,omitempty"`
	Stdout string  `json:"stdout,omitempty"`
	Ms     float64 `json:"ms"`
}

// dojoDecode reads one input into a Go value, or panics (reported as the
// case's error).
func dojoDecode(input map[string]json.RawMessage, name string, into any) {
	raw, ok := input[name]
	if !ok {
		panic(fmt.Sprintf("dojo: input %q missing", name))
	}
	if err := json.Unmarshal(raw, into); err != nil {
		panic(fmt.Sprintf("dojo: input %q: %v", name, err))
	}
}

func dojoList(values []int) *ListNode {
	dummy := &ListNode{}
	tail := dummy
	for _, v := range values {
		tail.Next = &ListNode{Val: v}
		tail = tail.Next
	}
	return dummy.Next
}

func dojoTree(values []*int) *TreeNode {
	if len(values) == 0 || values[0] == nil {
		return nil
	}
	root := &TreeNode{Val: *values[0]}
	queue := []*TreeNode{root}
	i := 1
	for q := 0; q < len(queue) && i < len(values); q++ {
		node := queue[q]
		if values[i] != nil {
			node.Left = &TreeNode{Val: *values[i]}
			queue = append(queue, node.Left)
		}
		i++
		if i < len(values) && values[i] != nil {
			node.Right = &TreeNode{Val: *values[i]}
			queue = append(queue, node.Right)
		}
		i++
	}
	return root
}

func dojoFromList(node *ListNode) []any {
	out := []any{}
	for node != nil {
		if len(out) >= dojoMaxNodes {
			panic("returned linked list is too long (cycle?)")
		}
		out = append(out, node.Val)
		node = node.Next
	}
	return out
}

func dojoFromTree(root *TreeNode) []any {
	out := []any{}
	queue := []*TreeNode{root}
	for q := 0; q < len(queue); q++ {
		if len(out) >= dojoMaxNodes {
			panic("returned tree is too large (cycle?)")
		}
		node := queue[q]
		if node == nil {
			out = append(out, nil)
			continue
		}
		out = append(out, node.Val)
		queue = append(queue, node.Left, node.Right)
	}
	for len(out) > 0 && out[len(out)-1] == nil {
		out = out[:len(out)-1]
	}
	return out
}

var dojoListType = reflect.TypeOf((*ListNode)(nil))
var dojoTreeType = reflect.TypeOf((*TreeNode)(nil))

// dojoJSON turns a returned value into plain JSON data: nodes become arrays
// and nil slices become empty ones (an empty answer, idiomatic in Go).
func dojoJSON(v reflect.Value) any {
	switch {
	case v.Type() == dojoListType:
		return dojoFromList(v.Interface().(*ListNode))
	case v.Type() == dojoTreeType:
		return dojoFromTree(v.Interface().(*TreeNode))
	case v.Kind() == reflect.Slice || v.Kind() == reflect.Array:
		out := make([]any, v.Len())
		for i := range out {
			out[i] = dojoJSON(v.Index(i))
		}
		return out
	case v.Kind() == reflect.Interface && v.IsNil():
		return nil
	default:
		return v.Interface()
	}
}

// dojoStack keeps the panic's frames that are in the solution.
func dojoStack() string {
	var keep []string
	lines := strings.Split(string(debug.Stack()), "\n")
	for i := 0; i+1 < len(lines); i++ {
		if strings.Contains(lines[i+1], "solution.go") {
			file := lines[i+1][strings.LastIndex(lines[i+1], "solution.go"):]
			if sp := strings.Index(file, " "); sp > 0 {
				file = file[:sp]
			}
			name := strings.TrimSpace(lines[i])
			if p := strings.LastIndex(name, "("); p > 0 {
				name = name[:p]
			}
			keep = append(keep, "  "+strings.TrimPrefix(name, "main.")+" ("+file+")")
		}
	}
	return strings.Join(keep, "\n")
}

func dojoWrite(path string, results []dojoResult) {
	data, err := json.Marshal(map[string]any{"results": results})
	if err != nil {
		panic(err)
	}
	if err := os.WriteFile(path, data, 0o644); err != nil {
		panic(err)
	}
}

func dojoProgress(path, step string) {
	_ = os.WriteFile(path, []byte(step), 0o644)
}

// dojoCapture runs f with os.Stdout sent to a temporary file and returns
// what was printed, capped.
func dojoCapture(f func()) string {
	tmp, err := os.CreateTemp("", "dojo-stdout")
	if err != nil {
		f()
		return ""
	}
	defer os.Remove(tmp.Name())
	real := os.Stdout
	os.Stdout = tmp
	defer func() { os.Stdout = real }()
	f()
	os.Stdout = real
	_, _ = tmp.Seek(0, io.SeekStart)
	buf := make([]byte, dojoStdoutCap+1)
	n, _ := io.ReadFull(tmp, buf)
	tmp.Close()
	out := string(buf[:min(n, dojoStdoutCap)])
	if n > dojoStdoutCap {
		out += "\n… (more output not shown)"
	}
	return strings.TrimRight(out, "\n")
}

// dojoRun runs every case through call (generated per question).
func dojoRun(call func(input map[string]json.RawMessage) any) {
	specPath, resultsPath := os.Args[1], os.Args[2]
	progressPath := resultsPath + ".progress"
	dojoProgress(progressPath, "load")
	data, err := os.ReadFile(specPath)
	if err != nil {
		panic(err)
	}
	var spec dojoSpec
	if err := json.Unmarshal(data, &spec); err != nil {
		panic(err)
	}
	results := []dojoResult{}
	for _, c := range spec.Cases {
		dojoProgress(progressPath, fmt.Sprint(c.Index))
		r := dojoResult{Index: c.Index}
		start := time.Now()
		r.Stdout = dojoCapture(func() {
			defer func() {
				if p := recover(); p != nil {
					r.Status = "error"
					r.Error = fmt.Sprint("panic: ", p)
					if stack := dojoStack(); stack != "" {
						r.Error += "\n" + stack
					}
				}
			}()
			got := dojoJSON(reflect.ValueOf(call(c.Input)))
			r.Status = "ok"
			r.Got = got
		})
		r.Ms = float64(time.Since(start).Microseconds()) / 1000
		if r.Status == "ok" && r.Got == nil {
			r.Got = json.RawMessage("null")
		}
		results = append(results, r)
		dojoWrite(resultsPath, results)
	}
	dojoProgress(progressPath, "done")
	dojoWrite(resultsPath, results)
}
