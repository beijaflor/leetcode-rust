// https://leetcode.com/submissions/detail/534136324/
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

const result = [];

function travarsal(node: Node | null, level: number) {
  if (node === null) return;

  if (result[level] === undefined) result[level] = [];
  result[level].push(node.val);

  node.children.forEach((child) => {
    travarsal(child, level + 1);
  });
}

function levelOrder(root: Node | null): number[][] {
  result.splice(0);
  travarsal(root, 0);
  return result;
}
