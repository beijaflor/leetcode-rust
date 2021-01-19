// https://leetcode.com/submissions/detail/437548839/

/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

type Path = "left" | "right";

function getTargetCopy(
  original: TreeNode | null,
  cloned: TreeNode | null,
  target: TreeNode | null
): TreeNode | null {
  const path = dig(original, target, []);

  if (path === false) return null;

  if (cloned === null) return null;

  let node: TreeNode = cloned;
  for (let i = 0; i < path.length; i++) {
    const p = path[i];
    if (p === "left" && node.left !== null) {
      node = node.left;
    } else if (p === "right" && node.right !== null) {
      node = node.right;
    }
  }
  return node;
}

function dig(
  current: TreeNode | null,
  target: TreeNode | null,
  path: Path[]
): Path[] | false {
  if (current !== null) {
    if (current.val === target.val) return path;

    if (current.left !== null) {
      const ret = dig(current.left, target, [...path, "left"]);
      if (ret !== false) return ret;
    }

    if (current.right !== null) {
      const ret = dig(current.right, target, [...path, "right"]);
      if (ret !== false) return ret;
    }
  }
  return false;
}
