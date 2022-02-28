/**
 * # 210. Course Schedule II
 *
 * There are a total of `numCourses` courses you have to take, labeled from `0` to
 * `numCourses - 1`. You are given an array `prerequisites` where `prerequisites[i] = [aᵢ, bᵢ]`
 * indicates that you __must__ take course `bᵢ` first if you want to take course `aᵢ`.
 * - For example, the pair `[0, 1]`, indicates that to take course `0` you have to first take
 * course `1`.
 * return _the ordering of courses you should take to finish all courses_. If there are many valid
 * answers, return __any__ of them. If it is impossible to finish all courses, return __an empty
 * array__.
 *
 * ## Constraints:
 * - `0 <= numCourses <= 2000`
 * - `0 <= prerequisites.length <= numCourses * (numCourses - 1)`
 * - `prerequisites[i].length == 2
 * - `0 <= aᵢ, bᵢ < numCourses`
 * - `aᵢ != bᵢ`
 * - All the pairs `[aᵢ, bᵢ]` are __distinct__.
 */

/**
 * @param {number} numCourses
 * @param {number[][]} prerequisites
 * @returns {number[]}
 */
function findOrder(numCourses: number, prerequisites: number[][]): number[] {
  const prereqGraph: Map<number, Array<number>> = new Map();
  const inDegree: Map<number, number> = new Map();

  for (let i = 0; i < numCourses; ++i) {
    prereqGraph.set(i, new Array());
    inDegree.set(i, 0);
  }

  for (let [course, prereq] of prerequisites) {
    let courseList = prereqGraph.get(prereq);
    if (courseList instanceof Array) {
      courseList.push(course);
    }

    let courseInDegree = inDegree.get(course);
    if (typeof courseInDegree === "number") {
      inDegree.set(course, courseInDegree + 1);
    }
  }

  const [topOrder, i] = sort(prereqGraph, inDegree);

  return i === numCourses ? topOrder : new Array();
}

function sort(
  prereqGraph: Map<number, Array<number>>,
  inDegree: Map<number, number>
): [Array<number>, number] {
  const order: Array<number> = [];
  let i = 0;

  const zeroDegree = [...inDegree.entries()]
    .filter(([_k, v]) => v === 0)
    .map(([k, _v]) => k);

  function sortHelper(value: number) {
    order.push(value);
    i += 1;
    const prereqCourses = prereqGraph.get(value);

    if (prereqCourses instanceof Array) {
      for (let course of prereqCourses) {
        let courseDegree = inDegree.get(course);

        if (typeof courseDegree === "number") {
          let newDegree = courseDegree - 1;
          inDegree.set(course, newDegree);

          if (newDegree === 0) {
            sortHelper(course);
          }
        }
      }
    }
  }

  for (let value of zeroDegree) {
    sortHelper(value);
  }

  return [order, i];
}

export { findOrder };
