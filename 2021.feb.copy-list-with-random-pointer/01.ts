// https://leetcode.com/submissions/detail/454723521/?
/**
 * Definition for Node.
 * class Node {
 *     val: number
 *     next: Node | null
 *     random: Node | null
 *     constructor(val?: number, next?: Node, random?: Node) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *         this.random = (random===undefined ? null : random)
 *     }
 * }
 */

function copyRandomList(head: Node | null): Node | null {
  if (!head) return null;

  let nodes: Node[] = [];
  let oldNodes: Node[] = []
  {
      let node = head
      while (node) {
          oldNodes.push(node);
          nodes.push(new Node(node.val))
          node = node.next
      }
  }

  oldNodes.forEach((node, index) => {
      const nextIndex = oldNodes.indexOf(node.next)
      // console.log("next:", nextIndex)
      if (nextIndex !== -1) {
          nodes[index].next = nodes[nextIndex]
      }

      const randomIndex = oldNodes.indexOf(node.random)
      // console.log("random:", randomIndex)
      if (randomIndex !== -1) {
          nodes[index].random = nodes[randomIndex]
      }
  })
  
  // console.log(nodes)
  // let test = nodes[0]
  // while (test) {
  //     console.dir(test, { depth: 1 })
  //     test = test.next
  // }
  return nodes[0]
};