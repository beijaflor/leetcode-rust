// https://leetcode.com/submissions/detail/463680173/
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

function getLength(node: ListNode): number {
  if (node.next === null) return 0;
  return getLength(node.next) + 1;
}

function getIntersectionNode(
  headA: ListNode | null,
  headB: ListNode | null
): ListNode | null {
  if (headA === null || headB === null) return null;
  const aLength = getLength(headA);
  const bLength = getLength(headB);

  if (aLength > bLength) {
    const diff = aLength - bLength;
    for (let i = 0; i < diff; i++) {
      headA = headA.next;
    }
  } else {
    const diff = bLength - aLength;
    for (let i = 0; i < diff; i++) {
      headB = headB.next;
    }
  }

  // console.log(headA);
  // console.log(headB);

  while (headA !== null && headA !== headB) {
    headA = headA.next;
    headB = headB.next;
  }
  return headA;
}
