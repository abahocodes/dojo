// 1001. Encode and Decode Strings (medium)
// /show to reread the problem · /test runs visible tests · /submit when done
package main

func encode(strs []string) string {
	return ""
}

func decode(s string) []string {
	return nil
}

// dojo calls this to run the tests; you don't need to change it.
func encodeAndDecode(strs []string) []string {
	encoded := encode(append([]string{}, strs...))
	other := make([]string, 0, len(strs)+1)
	for i := len(strs) - 1; i >= 0; i-- {
		other = append(other, strs[i])
	}
	encode(append(other, "dojo"))
	return decode(encoded)
}
