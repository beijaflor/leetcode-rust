// https://leetcode.com/submissions/detail/451563113/
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

function hasCycle(head: ListNode | null): boolean {
  if (!head) return false;
  let current = head;
  while (true) {
    if (!current.next) return false;
    current.val = 100_000 + 1;
    if (current.next.val === 100_000 + 1) return true;
    current = current.next;
  }
}
