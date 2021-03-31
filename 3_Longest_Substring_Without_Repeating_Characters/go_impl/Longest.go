package main

import "fmt"

func main() {
	fmt.Println(lengthOfLongestSubstring("abcab"))
}

func lengthOfLongestSubstring(s string) int {
	i, j := 0, 0
	if len(s) == 1 {
		return 1
	}
	maxLen := 0
	jump := false
	for i < len(s) {
		jump = false
		m := make(map[string]int)
		for j = i + 1; j < len(s); j++ {
			l, ok := m[string(s[j])]
			if ok {
				if l > i {
					i = l
					jump = true
					break
				}
			} else {
				m[string(s[j])] = j
			}
			maxLen = max(j-i, maxLen)
		}
		if !jump {
			i++
		}
	}
	return maxLen
}

func max(i, j int) int {
	if i > j {
		return i
	} else {
		return j
	}
}
