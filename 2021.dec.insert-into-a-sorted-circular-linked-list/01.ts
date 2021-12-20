// https://leetcode.com/submissions/detail/597870881/
/**
 * Definition for node.
 * class Node {
 *     val: number
 *     next: Node | null
 *     constructor(val?: number, next?: Node) {
 *         this.val = (val===undefined ? 0 : val);
 *         this.next = (next===undefined ? null : next);
 *     }
 * }
 */

function insert(head: Node | null, insertVal: number): Node | null {
    // console.log(head);
    if (head === null) {
        let node = new Node(insertVal)
        node.next = node;
        return node;
    }
    let prev = head;
    let current = head.next;
    let toInsert = false;

    while (true) {
        if (prev.val <= insertVal && insertVal <= current.val) {
            toInsert = true;
        } else if (prev.val > current.val) {
            if (insertVal >= prev.val || insertVal <= current.val) {
                toInsert = true;
            }
        }
        
        if (toInsert) {
            prev.next = new Node(insertVal, current);
            return head;
        }
        
        prev = current;
        current = current.next;
        
        if (prev === head) break;
    }
    
    prev.next = new Node(insertVal, current);
    return head;
}