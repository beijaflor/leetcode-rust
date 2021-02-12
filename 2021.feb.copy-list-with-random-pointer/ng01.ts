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
  let oldNodes: Node[] = []
  let oldRandoms: Node[] = []
  let nodes: Node[] = []
  let randoms: Node[] = []
  
  let node = head;
  while (node) {
      // console.log("val", node.val)
      console.log("---")
      let test = nodes[0]
      while (test) {
          console.dir(`${node.val}, ${test.random && test.random .val}`, { depth: 1 })
          test = test.next
      }
      // console.log("nodes", nodes)
      // console.log("randoms", randoms)
      // console.log("oldNodes", oldNodes)
      // console.log("oldRandoms", oldRandoms)

      let newNode
      const randomIndex = oldRandoms.indexOf(node)
      if (randomIndex !== -1) {
          console.log("yo-ho-")
          newNode = randoms[randomIndex]
      } else {
          console.log("ya-ha-")
          newNode = new Node(node.val)
      }

      if (node.random) {
          const index = oldNodes.indexOf(node.random)
          // console.log(index)
          if (index !== -1) {
              newNode.random = nodes[index];
          } else if (node.random === node) {
              console.log("yo-ho-")
          } else {
              const newRandomNode = new Node(node.random.val)
              randoms.push(newRandomNode)
              newNode.random = randoms[randoms.length - 1];
              oldRandoms.push(node.random)
          }
      }

      if (nodes.length > 0) {
          nodes[nodes.length - 1].next = newNode
      }
      nodes.push(newNode)

//         if (node.random === node) {
//             nodes[nodes.length - 1].random = nodes[nodes.length - 1];
//         }

      if (randomIndex !== -1) {
          console.log("yo-ho-")
          randoms[randomIndex] = nodes[nodes.length - 1]
      }

      oldNodes.push(node)
      node = node.next;
  }

  // console.log(nodes[0])
  // let test = nodes[0]
  // while (test) {
  //     console.dir(test, { depth: 1 })
  //     test = test.next
  // }
  return nodes[0]
};