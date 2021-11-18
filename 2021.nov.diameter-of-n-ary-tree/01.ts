// https://leetcode.com/submissions/detail/587571200/
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

function dig(node: Node): [number, number] {
    let max = 0;
    let height1 = 0;
    let height2 = 0;
    node.children.forEach(child => {
        const [_max, _height] = dig(child);
        max = Math.max(max, _max)
        if (_height > height1) {
            height2 = height1
            height1 = _height
        } else if (_height > height2) {
            height2 = _height;
        }
    })

    return [Math.max(max, height1 + height2), height1 + 1]
}

function diameter(root: Node): number {
    return dig(root)[0]
};