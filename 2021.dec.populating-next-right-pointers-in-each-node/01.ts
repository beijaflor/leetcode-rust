// https://leetcode.com/submissions/detail/609001672/
/**
 * Definition for Node.
 * class Node {
 *     val: number
 *     left: Node | null
 *     right: Node | null
 *     next: Node | null
 *     constructor(val?: number, left?: Node, right?: Node, next?: Node) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function connect(root: Node | null): Node | null {
    // return root;
    if (root === null) {
        return null;
    }

    let queue = [root];
    while (true) {
        console.log(queue.map(x => x.val));
        let next = null;
        let nextQueue = [];
        queue.forEach(node => {
            // console.log(node);
            node.next = next;
            next = node;
            nextQueue.push(node.right);
            nextQueue.push(node.left);
        });
        queue = nextQueue;
        if (queue[0] === null) break
    }    
    
    return root;
};