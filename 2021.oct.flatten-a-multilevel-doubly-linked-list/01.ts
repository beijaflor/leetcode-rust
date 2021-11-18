// https://leetcode.com/submissions/detail/580017221/
/**
 * Definition for node.
 * class Node {
 *     val: number
 *     prev: Node | null
 *     next: Node | null
 *     child: Node | null
 *     constructor(val?: number, prev? : Node, next? : Node, child? : Node) {
 *         this.val = (val===undefined ? 0 : val);
 *         this.prev = (prev===undefined ? null : prev);
 *         this.next = (next===undefined ? null : next);
 *         this.child = (child===undefined ? null : child);
 *     }
 * }
 */

function flatten(head: Node | null): Node | null {
    let node = head;
    let node_stack: Node[] = [];
    while (node) {
        if (node.child) {
            if (node.next !== null) {
                node_stack.push(node.next);
            }
            node.next = node.child;
            node.child = null;
        }
        if (node.next === null && node_stack.length !== 0) {
            node.next = node_stack.pop();
        }
        if (node.next !== null) {
            node.next.prev = node;
        }
        node = node.next;
    }    
    // console.dir(head, { depth: null})
    return head;
};
