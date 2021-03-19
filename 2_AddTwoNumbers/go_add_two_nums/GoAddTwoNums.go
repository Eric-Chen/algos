package main

import "fmt"

func main() {
	l1 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 0,
				Next: &ListNode{
					Val: 1,
				},
			},
		},
	}
	l2 := &ListNode{
		Val: 3,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 0,
				Next: &ListNode{
					Val: 9,
					Next: &ListNode{
						Val: 5,
					},
				},
			},
		},
	}
	l := AddTwo(l1, l2)
	for ; l != nil; l = l.Next {
		fmt.Println(l.Val)
	}
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func AddTwo(l1 *ListNode, l2 *ListNode) *ListNode {
	var l *ListNode = &ListNode{}
	r := l
	for {
		v1, v2 := 0, 0
		if l1 != nil {
			v1 = l1.Val
		}
		if l2 != nil {
			v2 = l2.Val
		}
		v := v1 + v2
		if v > 9 {
			l.Val = l.Val + (v - 10)
			l.Next = &ListNode{
				Val: 1,
			}
		} else {
			l.Val = l.Val + v
		}

		if l.Val > 9 {
			l.Val = l.Val - 10
			l.Next = &ListNode{
				Val: 1,
			}
		}

		if l1 != nil {
			l1 = l1.Next
		}
		if l2 != nil {
			l2 = l2.Next
		}

		if l1 == nil && l2 == nil {
			break
		} else {
			if l.Next == nil {
				l.Next = &ListNode{}
			}
			l = l.Next
		}
	}

	return r
}
