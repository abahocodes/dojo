// 1003. Time Based Key-Value Store (medium)
// /show to reread the problem · /test runs visible tests · /submit when done
package main

type TimeMap struct {
}

func Constructor() TimeMap {
	return TimeMap{}
}

func (this *TimeMap) Set(key string, value string, timestamp int) {

}

func (this *TimeMap) Get(key string, timestamp int) string {
	return ""
}

// dojo calls this to run the tests; you don't need to change it.
func timeMap(ops []string, keys []string, values []string, timestamps []int) []string {
	store := Constructor()
	out := []string{}
	for i, op := range ops {
		if op == "set" {
			store.Set(keys[i], values[i], timestamps[i])
		} else {
			out = append(out, store.Get(keys[i], timestamps[i]))
		}
	}
	return out
}
