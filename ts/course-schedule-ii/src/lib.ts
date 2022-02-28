/**
 * # 210. Course Schedule II
 *
 * There are a total of `numCourses` courses you have to take, labeled from `0` to
 * `numCourses - 1`. You are given an array `prerequisites` where `prerequisites[i] = [aᵢ, bᵢ]`
 * indicates that you **must** take course `bᵢ` first if you want to take course `aᵢ`.
 * - For example, the pair `[0, 1]`, indicates that to take course `0` you have to first take
 * course `1`.
 * return *the ordering of courses you should take to finish all courses*. If there are many valid
 * answers, return **any** of them. If it is impossible to finish all courses, return **an empty
 * array**.
 *
 * ## Constraints:
 * - `0 <= numCourses <= 2000`
 * - `0 <= prerequisites.length <= numCourses * (numCourses - 1)`
 * - `prerequisites[i].length == 2
 * - `0 <= aᵢ, bᵢ < numCourses`
 * - `aᵢ != bᵢ`
 * - All the pairs `[aᵢ, bᵢ]` are **distinct**.
 */

export * as v1 from "./v1";
