// https://leetcode.com/submissions/detail/453639616/
/**
 * // This is the Iterator's API interface.
 * // You should not implement it, or speculate about its implementation
 * class Iterator {
 *      hasNext(): boolean {}
 *
 *      next(): number {}
 * }
 */

class PeekingIterator {
  iterator: Iterator;

  _next: number;
  _hasNext: boolean;

  constructor(iterator: Iterator) {
    this.iterator = iterator;
    this._hasNext = iterator.hasNext();
    this._next = iterator.next();
  }

  peek(): number {
    return this._next;
  }

  next(): number {
    const _next = this._next;
    this._hasNext = this.iterator.hasNext();
    this._next = this.iterator.next();
    return _next;
  }

  hasNext(): boolean {
    return this._hasNext;
  }
}

/**
 * Your PeekingIterator object will be instantiated and called as such:
 * var obj = new PeekingIterator(iterator)
 * var param_1 = obj.peek()
 * var param_2 = obj.next()
 * var param_3 = obj.hasNext()
 */
