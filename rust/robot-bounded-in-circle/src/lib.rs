//! # 1041. Robot Bounded In Circle
//!
//! On an infinite plane, a robot initially stands at `(0, 0)` and faces north. The robot can
//! receive one of three instructions:
//! - `"G"`: go straight 1 unit;
//! - `"L"`: turn 90 degrees to the left;
//! - `"R"`: turn 90 degrees to the right.
//!
//! The robot performs the `instructions` given in order, and repeats them forever.
//!
//! Return `true` if and if there exists a circle in the plane such that the robot never leaves the
//! circle.
//!
//! ## Constraints:
//! - `1 <= instructions.length <= 100`
//! - `instructions[i]` is `'G'`, `'L'`, or `'R'`.

struct Solution;

impl Solution {
    fn is_robot_bounded(instructions: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut dir = 0;

        for c in instructions.chars() {
            match c {
                'G' => {
                    match dir {
                        0 => {
                            y += 1;
                        }
                        1 => {
                            x += 1;
                        }
                        2 => {
                            y -= 1;
                        }
                        3 => {
                            x -= 1;
                        }
                        _ => {
                            // TODO: error
                            return false;
                        }
                    }
                }
                'L' => {
                    dir = (dir + 3) % 4;
                }
                'R' => {
                    dir = (dir + 1) % 4;
                }
                _ => {
                    // TODO: error
                    return false;
                }
            }
        }

        (x == 0 && y == 0) || dir != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let instructions: String = "GGLLGG".into();
        let output = Solution::is_robot_bounded(instructions);

        assert!(output);
    }

    #[test]
    fn example_two() {
        let instructions: String = "GG".into();
        let output = Solution::is_robot_bounded(instructions);

        assert!(!output);
    }

    #[test]
    fn example_three() {
        let instructions: String = "GL".into();
        let output = Solution::is_robot_bounded(instructions);

        assert!(output);
    }
}
