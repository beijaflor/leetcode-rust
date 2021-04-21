// https://leetcode.com/submissions/detail/483229660/

/**
 * Definition for node.
 * class Node {
 *     val: number
 *     children: Node[]
 *     constructor(val?: number) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.children = []
 *     }
 * }
 */

function preorder(root: Node | null): number[] {
  if (root === null) return [];
  const children_vals = root.children.flatMap((node) => preorder(node));
  return [root.val, ...children_vals];
}
