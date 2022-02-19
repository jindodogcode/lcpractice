/**
 * <h1>Course Schedule II</h1>
 * <p>
 * There are a total of <code>numCourses</code> courses you have to take,
 * labeled from <code>0</code> to <code>numCourses - 1</code>. You are given an
 * array <code>prerequisites</code> where
 * <code>prerequisites[i] = [ai, bi]</code> indicates that you <b>must</b> take
 * course <code>bi</code> first if you want to take course <code>ai</code>.
 * </p>
 *
 * <ul>
 * <li>For Example, the pair <code>[0, 1]</code>, indicates that to take course
 * <code>0</code> you have to first take course <code>1</code>.</li>
 * </ul>
 *
 * </p>
 * Return <em>the ordering of courses you should take to finish all
 * courses</em>. If there are many valid answers, return <b>any</b> of them. If
 * it is impossible to finish all courses, return <b>an empty array</b>.
 *
 */
package dev.mkennedy.coursescheduleii;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;
import java.util.Queue;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Library {
    public static int[] findOrder(int numCourses, int[][] prerequisites) {
        List<List<Integer>> graph = new ArrayList<>(numCourses);
        List<Integer> inDegree = new ArrayList<>(numCourses);

        for (int i = 0; i < numCourses; ++i) {
            graph.add(new ArrayList<>());
            inDegree.add(0);
        }

        for (int[] edge : prerequisites) {
            int src = edge[1];
            int dest = edge[0];
            graph.get(src).add(dest);
        }

        Queue<Integer> queue = IntStream.range(0, numCourses)
                .filter(i -> inDegree.get(i) == 0).boxed()
                .collect(Collectors.toCollection(ArrayDeque::new));
        List<Integer> order = new ArrayList<>(numCourses);
        int count = 0;

        while (!queue.isEmpty()) {
            int course = queue.poll();
            order.add(course);
            ++count;

            for (int dep : graph.get(course)) {
                inDegree.set(dep, inDegree.get(dep) - 1);

                if (inDegree.get(dep) == 0) {
                    queue.add(dep);
                }
            }
        }

        if (count == numCourses) {
            return order.stream().mapToInt(i -> i).toArray();
        } else {
            return new int[] {};
        }
    }
}
