mod utils;
mod types;

pub use types::{Face};
use utils::{rot};

pub(crate) struct Cube {
    start_time: i64,
    last_step: Vec<char>,
    front: Face,
    back: Face,
    left: Face,
    right: Face,
    up: Face,
    down: Face,
}
impl Cube {
    pub fn new() -> Cube {
        let start_time = 0;
        let last_step = Vec::new();
        Cube {
            start_time,
            last_step,
            front: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
            back: [[10, 11, 12], [13, 14, 15], [16, 17, 18]],
            left: [[19, 20, 21], [22, 23, 24], [25, 26, 27]],
            right: [[28, 29, 30], [31, 32, 33], [34, 35, 36]],
            up: [[37, 38, 39], [40, 41, 42], [43, 44, 45]],
            down: [[46, 47, 48], [49, 50, 51], [52, 53, 54]],
        }
    }
    pub fn reset(&mut self){
        self.front = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        self.back = [[10, 11, 12], [13, 14, 15], [16, 17, 18]];
        self.left = [[19, 20, 21], [22, 23, 24], [25, 26, 27]];
        self.right = [[28, 29, 30], [31, 32, 33], [34, 35, 36]];
        self.up = [[37, 38, 39], [40, 41, 42], [43, 44, 45]];
        self.down = [[46, 47, 48], [49, 50, 51], [52, 53, 54]];
    }

    pub fn rot_front(&mut self, rev: bool){
        let temp_left = self.left;
        let temp_up = self.up[2];
        let temp_right = self.right;
        let temp_down = self.down[0];
        if rev {
            self.front = rot(self.front, rev);
            for i in 0..3 {
                self.left[i][2] = temp_up[2-i];
                self.right[i][0] = temp_down[2-i];
                self.up[2][i] = temp_right[i][0];
                self.down[0][i] = temp_left[i][2];
            }
        }
        else {
            self.front = rot(self.front, rev);
            for i in 0..3 {
                self.left[i][2] = temp_down[i];
                self.right[i][0] = temp_up[i];
                self.up[2][i] = temp_left[2-i][2];
                self.down[0][i] = temp_right[2-i][0];
            }
        }
    }
    pub fn rot_back(&mut self, rev: bool){
        let temp_left = self.left;
        let temp_up = self.up[0];
        let temp_right = self.right;
        let temp_down = self.down[2];
        if rev {
            self.back = rot(self.back, rev);
            for i in 0..3 {
                self.left[i][0] = temp_down[i];
                self.right[i][2] = temp_up[i];
                self.up[0][i] = temp_left[2-i][0];
                self.down[2][i] = temp_right[2-i][2];
            }
        }
        else {
            self.back = rot(self.back, rev);
            for i in 0..3 {
                self.left[i][0] = temp_up[2-i];
                self.right[i][2] = temp_down[2-i];
                self.up[0][i] = temp_right[i][2];
                self.down[2][i] = temp_left[i][0];
            }
        }
    }
    pub fn rot_left(&mut self, rev: bool){
        let temp_front = self.front;
        let temp_up = self.up;
        let temp_back = self.back;
        let temp_down = self.down;
        if rev {
            self.left = rot(self.left, rev);
            for i in 0..3 {
                self.front[i][0] = temp_down[i][0];
                self.back[i][2] = temp_up[2-i][0];
                self.down[i][0] = temp_back[2-i][2];
                self.up[i][0] = temp_front[i][0];
            }
        }
        else {
            self.left = rot(self.left, rev);
            for i in 0..3 {
                self.front[i][0] = temp_up[i][0];
                self.up[i][0] = temp_back[2-i][2];
                self.back[i][2] = temp_down[2-i][0];
                self.down[i][0] = temp_front[i][0];
            }
        }
    }
    pub fn rot_right(&mut self, rev: bool){
        let temp_front = self.front;
        let temp_up = self.up;
        let temp_back = self.back;
        let temp_down = self.down;
        if rev {
            self.right = rot(self.right, rev);
            for i in 0..3 {
                self.front[i][2] = temp_up[i][2];
                self.back[i][0] = temp_down[2-i][2];
                self.down[i][2] = temp_front[i][2];
                self.up[i][2] = temp_back[2-i][0];
            }
        }
        else {
            self.right = rot(self.right, rev);
            for i in 0..3 {
                self.front[i][2] = temp_down[i][2];
                self.up[i][2] = temp_front[i][2];
                self.back[i][0] = temp_up[2-i][2];
                self.down[i][2] = temp_back[2-i][0];
            }
        }
    }
    pub fn rot_up(&mut self, rev: bool){
        let temp_front = self.front;
        let temp_left = self.left;
        let temp_back = self.back;
        let temp_right = self.right;
        if rev {
            self.up = rot(self.up, rev);
            for i in 0..3 {
                self.front[0][i] = temp_left[0][i];
                self.left[0][i] = temp_back[0][i];
                self.back[0][i] = temp_right[0][i];
                self.right[0][i] = temp_front[0][i];
            }
        }
        else {
            self.up = rot(self.up, rev);
            for i in 0..3 {
                self.front[0][i] = temp_right[0][i];
                self.left[0][i] = temp_front[0][i];
                self.back[0][i] = temp_left[0][i];
                self.right[0][i] = temp_back[0][i];
            }
        }
    }
    pub fn rot_down(&mut self, rev: bool){
        let temp_front = self.front;
        let temp_left = self.left;
        let temp_back = self.back;
        let temp_right = self.right;
        if rev {
            self.down = rot(self.down, rev);
            for i in 0..3 {
                self.front[2][i] = temp_right[2][i];
                self.left[2][i] = temp_front[2][i];
                self.back[2][i] = temp_left[2][i];
                self.right[2][i] = temp_back[2][i];
            }
        }
        else {
            self.down = rot(self.down, rev);
            for i in 0..3 {
                self.front[2][i] = temp_left[2][i];
                self.left[2][i] = temp_back[2][i];
                self.back[2][i] = temp_right[2][i];
                self.right[2][i] = temp_front[2][i];
            }
        }

    }

    pub fn is_solved(&self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                match self.front[i][j] {
                    1..=9 => (),
                    _ => return false,
                }
                match self.back[i][j] {
                    10..=18 => (),
                    _ => return false,
                }
                match self.left[i][j] {
                    19..=27 => (),
                    _ => return false,
                }
                match self.right[i][j] {
                    28..=36 => (),
                    _ => return false,
                }
                match self.up[i][j] {
                    37..=45 => (),
                    _ => return false,
                }
                match self.down[i][j] {
                    46..=54 => (),
                    _ => return false,
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_rot_front() {
        let mut cube = Cube::new();
        cube.rot_front(false);

        // down: [
        // [46, 47, 48],
        // [49, 50, 51],
        // [52, 53, 54]],
        assert_eq!(cube.left, [
            [19, 20, 46],
            [22, 23, 47],
            [25, 26, 48]]);
        // left: [
        // [19, 20, 21],
        // [22, 23, 24],
        // [25, 26, 27]],
        assert_eq!(cube.up, [
            [37, 38, 39],
            [40, 41, 42],
            [27, 24, 21]]);
        // up: [
        // [37, 38, 39],
        // [40, 41, 42],
        // [43, 44, 45]],
        assert_eq!(cube.right, [
            [43, 29, 30],
            [44, 32, 33],
            [45, 35, 36]]);
        // right: [
        // [28, 29, 30],
        // [31, 32, 33],
        // [34, 35, 36]],
        assert_eq!(cube.down, [
            [34, 31, 28],
            [49, 50, 51],
            [52, 53, 54]]);
    }
    #[test]
    fn it_works_rot_front_rev() {
        let mut cube = Cube::new();
        cube.rot_front(true);
        // up: [
        // [37, 38, 39],
        // [40, 41, 42],
        // [43, 44, 45]],
        assert_eq!(cube.left, [
            [19, 20, 45],
            [22, 23, 44],
            [25, 26, 43]]);
        // right: [
        // [28, 29, 30],
        // [31, 32, 33],
        // [34, 35, 36]],
        assert_eq!(cube.up, [
            [37, 38, 39],
            [40, 41, 42],
            [28, 31, 34]]);
        // down: [
        // [46, 47, 48],
        // [49, 50, 51],
        // [52, 53, 54]],
        assert_eq!(cube.right, [
            [48, 29, 30],
            [47, 32, 33],
            [46, 35, 36]]);
        // left: [
        // [19, 20, 21],
        // [22, 23, 24],
        // [25, 26, 27]],
        assert_eq!(cube.down, [
            [21, 24, 27],
            [49, 50, 51],
            [52, 53, 54]]);
    }

    #[test]
    fn it_works_rot_back() {
        let mut cube = Cube::new();
        cube.rot_back(false);
        // up: [
        // [37, 38, 39],
        // [40, 41, 42],
        // [43, 44, 45]],
        assert_eq!(cube.left, [
            [39, 20, 21],
            [38, 23, 24],
            [37, 26, 27]]);
        // right: [
        // [28, 29, 30],
        // [31, 32, 33],
        // [34, 35, 36]],
        assert_eq!(cube.up, [
            [30, 33, 36],
            [40, 41, 42],
            [43, 44, 45]]);
        // down: [
        // [46, 47, 48],
        // [49, 50, 51],
        // [52, 53, 54]],
        assert_eq!(cube.right, [
            [28, 29, 54],
            [31, 32, 53],
            [34, 35, 52]]);
        // left: [
        // [19, 20, 21],
        // [22, 23, 24],
        // [25, 26, 27]],
        assert_eq!(cube.down, [
            [46, 47, 48],
            [49, 50, 51],
            [19, 22, 25]]);
    }
    #[test]
    fn it_works_rot_back_rev() {
        let mut cube = Cube::new();
        cube.rot_back(true);
        // down: [
        // [46, 47, 48],
        // [49, 50, 51],
        // [52, 53, 54]],
        assert_eq!(cube.left, [
            [52, 20, 21],
            [53, 23, 24],
            [54, 26, 27]]);
        // left: [
        // [19, 20, 21],
        // [22, 23, 24],
        // [25, 26, 27]],
        assert_eq!(cube.up, [
            [25, 22, 19],
            [40, 41, 42],
            [43, 44, 45]]);
        // up: [
        // [37, 38, 39],
        // [40, 41, 42],
        // [43, 44, 45]],
        assert_eq!(cube.right, [
            [28, 29, 37],
            [31, 32, 38],
            [34, 35, 39]]);
        // right: [
        // [28, 29, 30],
        // [31, 32, 33],
        // [34, 35, 36]],
        assert_eq!(cube.down, [
            [46, 47, 48],
            [49, 50, 51],
            [36, 33, 30]]);
    }
    #[test]
    fn it_works_rot_left() {
        let mut cube = Cube::new();
        cube.rot_left(false);
        // back: [
        // [10, 11, 12],
        // [13, 14, 15],
        // [16, 17, 18]],
        assert_eq!(cube.up, [
            [18, 38, 39],
            [15, 41, 42],
            [12, 44, 45]]);
        // front: [
        // [1, 2, 3],
        // [4, 5, 6],
        // [7, 8, 9]],
        assert_eq!(cube.down, [
            [1, 47, 48],
            [4, 50, 51],
            [7, 53, 54]]);
        // up: [
        // [37, 38, 39],
        // [40, 41, 42],
        // [43, 44, 45]],
        assert_eq!(cube.front, [
            [37, 2, 3],
            [40, 5, 6],
            [43, 8, 9]]);
        // down: [
        // [46, 47, 48],
        // [49, 50, 51],
        // [52, 53, 54]],
        assert_eq!(cube.back, [
            [10, 11, 52],
            [13, 14, 49],
            [16, 17, 46]]);
    }

    #[test]
    fn it_works_rot_left_rev() {
        let mut cube = Cube::new();
        cube.rot_left(true);
        // front: [
        // [1, 2, 3],
        // [4, 5, 6],
        // [7, 8, 9]],
        assert_eq!(cube.up, [
            [1, 38, 39],
            [4, 41, 42],
            [7, 44, 45]]);
        // back: [
        // [10, 11, 12],
        // [13, 14, 15],
        // [16, 17, 18]],
        assert_eq!(cube.down, [
            [18, 47, 48],
            [15, 50, 51],
            [12, 53, 54]]);
        // down: [
        // [46, 47, 48],
        // [49, 50, 51],
        // [52, 53, 54]],
        assert_eq!(cube.front, [
            [46, 2, 3],
            [49, 5, 6],
            [52, 8, 9]]);
        // up: [
        // [37, 38, 39],
        // [40, 41, 42],
        // [43, 44, 45]],
        assert_eq!(cube.back, [
            [10, 11, 43],
            [13, 14, 40],
            [16, 17, 37]]);
    }
    #[test]
    fn it_works_rot_right() {
        let mut cube = Cube::new();
        cube.rot_right(false);
        // front: [
        // [1, 2, 3],
        // [4, 5, 6],
        // [7, 8, 9]],
        assert_eq!(cube.up, [
            [37, 38, 3],
            [40, 41, 6],
            [43, 44, 9]]);
        // back: [
        // [10, 11, 12],
        // [13, 14, 15],
        // [16, 17, 18]],
        assert_eq!(cube.down, [
            [46, 47, 16],
            [49, 50, 13],
            [52, 53, 10]]);
        // down: [
        // [46, 47, 48],
        // [49, 50, 51],
        // [52, 53, 54]],
        assert_eq!(cube.front, [
            [1, 2, 48],
            [4, 5, 51],
            [7, 8, 54]]);
        // up: [
        // [37, 38, 39],
        // [40, 41, 42],
        // [43, 44, 45]],
        assert_eq!(cube.back, [
            [45, 11, 12],
            [42, 14, 15],
            [39, 17, 18]]);
    }
    #[test]
    fn it_works_rot_right_rev() {
        let mut cube = Cube::new();
        cube.rot_right(true);
        // back: [
        // [10, 11, 12],
        // [13, 14, 15],
        // [16, 17, 18]],
        assert_eq!(cube.up, [
            [37, 38, 16],
            [40, 41, 13],
            [43, 44, 10]]);
        // front: [
        // [1, 2, 3],
        // [4, 5, 6],
        // [7, 8, 9]],
        assert_eq!(cube.down, [
            [46, 47, 3],
            [49, 50, 6],
            [52, 53, 9]]);
        // up: [
        // [37, 38, 39],
        // [40, 41, 42],
        // [43, 44, 45]],
        assert_eq!(cube.front, [
            [1, 2, 39],
            [4, 5, 42],
            [7, 8, 45]]);
        // down: [
        // [46, 47, 48],
        // [49, 50, 51],
        // [52, 53, 54]],
        assert_eq!(cube.back, [
            [54, 11, 12],
            [51, 14, 15],
            [48, 17, 18]]);
    }
    #[test]
    fn it_works_rot_up() {
        let mut cube = Cube::new();
        cube.rot_up(false);
        // right: [
        // [28, 29, 30],
        // [31, 32, 33],
        // [34, 35, 36]],
        assert_eq!(cube.front, [
            [28, 29, 30],
            [4, 5, 6],
            [7, 8, 9]]);
        // left: [
        // [19, 20, 21],
        // [22, 23, 24],
        // [25, 26, 27]],
        assert_eq!(cube.back, [
            [19, 20, 21],
            [13, 14, 15],
            [16, 17, 18]]);
        // front: [
        // [1, 2, 3],
        // [4, 5, 6],
        // [7, 8, 9]],
        assert_eq!(cube.left, [
            [1, 2, 3],
            [22, 23, 24],
            [25, 26, 27]]);
        // back: [
        // [10, 11, 12],
        // [13, 14, 15],
        // [16, 17, 18]],
        assert_eq!(cube.right, [
            [10, 11, 12],
            [31, 32, 33],
            [34, 35, 36]]);
    }
    #[test]
    fn it_works_rot_up_rev() {
        let mut cube = Cube::new();
        cube.rot_up(true);
        // left: [
        // [19, 20, 21],
        // [22, 23, 24],
        // [25, 26, 27]],
        assert_eq!(cube.front, [
            [19, 20, 21],
            [4, 5, 6],
            [7, 8, 9]]);
        // right: [
        // [28, 29, 30],
        // [31, 32, 33],
        // [34, 35, 36]],
        assert_eq!(cube.back, [
            [28, 29, 30],
            [13, 14, 15],
            [16, 17, 18]]);
        // back: [
        // [10, 11, 12],
        // [13, 14, 15],
        // [16, 17, 18]],
        assert_eq!(cube.left, [
            [10, 11, 12],
            [22, 23, 24],
            [25, 26, 27]]);
        // front: [
        // [1, 2, 3],
        // [4, 5, 6],
        // [7, 8, 9]],
        assert_eq!(cube.right, [
            [1, 2, 3],
            [31, 32, 33],
            [34, 35, 36]]);
    }
    #[test]
    fn it_works_rot_down() {
        let mut cube = Cube::new();
        cube.rot_down(false);
        // left: [
        // [19, 20, 21],
        // [22, 23, 24],
        // [25, 26, 27]],
        assert_eq!(cube.front, [
            [1, 2, 3],
            [4, 5, 6],
            [25, 26, 27]]);
        // right: [
        // [28, 29, 30],
        // [31, 32, 33],
        // [34, 35, 36]],
        assert_eq!(cube.back, [
            [10, 11, 12],
            [13, 14, 15],
            [34, 35, 36]]);
        // back: [
        // [10, 11, 12],
        // [13, 14, 15],
        // [16, 17, 18]],
        assert_eq!(cube.left, [
            [19, 20, 21],
            [22, 23, 24],
            [16, 17, 18]]);
        // front: [
        // [1, 2, 3],
        // [4, 5, 6],
        // [7, 8, 9]],
        assert_eq!(cube.right, [
            [28, 29, 30],
            [31, 32, 33],
            [7, 8, 9]]);
    }
    #[test]
    fn it_works_rot_down_rev() {
        let mut cube = Cube::new();
        cube.rot_down(true);
        // right: [
        // [28, 29, 30],
        // [31, 32, 33],
        // [34, 35, 36]],
        assert_eq!(cube.front, [
            [1, 2, 3],
            [4, 5, 6],
            [34, 35, 36]]);
        // left: [
        // [19, 20, 21],
        // [22, 23, 24],
        // [25, 26, 27]],
        assert_eq!(cube.back, [
            [10, 11, 12],
            [13, 14, 15],
            [25, 26, 27]]);
        // front: [
        // [1, 2, 3],
        // [4, 5, 6],
        // [7, 8, 9]],
        assert_eq!(cube.left, [
            [19, 20, 21],
            [22, 23, 24],
            [7, 8, 9]]);
        // back: [
        // [10, 11, 12],
        // [13, 14, 15],
        // [16, 17, 18]],
        assert_eq!(cube.right, [
            [28, 29, 30],
            [31, 32, 33],
            [16, 17, 18]]);
    }
    #[test]
    fn it_works_is_solved() {
        let mut cube = Cube::new();
        cube.rot_back(false);
        cube.rot_back(true);
        assert_eq!(cube.is_solved(), true);
        cube.rot_back(true);
        assert_eq!(cube.is_solved(), false);
    }
}