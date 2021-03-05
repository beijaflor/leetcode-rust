// https://leetcode.com/submissions/detail/463676098/
/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function getIntersectionNode(
  headA: ListNode | null,
  headB: ListNode | null
): ListNode | null {
  if (headA === null || headB === null) return null;
  let a = headA;
  let b = headB;
  while (a !== b) {
    if (a.next === null && b.next === null) return null;
    a = a.next === null ? headA : a.next;
    b = b.next === null ? headB : b.next;
  }
  return a;
}
