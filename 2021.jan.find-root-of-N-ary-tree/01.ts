// https://leetcode.com/submissions/detail/440560335/
/**
 * Definition for Node.
 * class Node {
 *     val: number
 *     children: Node[]
 *     constructor(val?: number, children?: Node[]) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.children = (children===undefined ? [] : children)
 *     }
 * }
 */

function findRoot(tree: Node[]): Node | null {
  const children = tree.reduce((acc, current) => {
      return [
          ...acc,
          ...current.children.map(node => node.val)
      ]},
      [] as number[])
  const index = tree.map(node => node.val).findIndex(x => !children.includes(x))
  return tree[index!!]
};
