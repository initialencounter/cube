use crate::types::Face;

pub fn rot(face: Face, rev: bool) -> Face {
    let mut result_face = [[0; 3]; 3];
    if rev {
        let mut i_ = 0;
        for i in 0..3 {
            let mut j_ = 0;
            for j in (0..3).rev() {
                result_face[j_][i_] = face[i][j];
                j_ += 1;
            }
            i_ += 1;
        }
    }
    else {
        let mut i_ = 0;
        for i in (0..3).rev() {
            let mut j_ = 0;
            for j in 0..3 {
                result_face[j_][i_] = face[i][j];
                j_ += 1;
            }
            i_ += 1;
        }
    }
    result_face
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let face = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ];
        let result = rot(face, false);
        assert_eq!(result, [
            [7, 4, 1],
            [8, 5, 2],
            [9, 6, 3]
        ]);
    }

    #[test]
    fn it_works_rev() {
        let face = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ];
        let result = rot(face, true);
        assert_eq!(result, [
            [3, 6, 9],
            [2, 5, 8],
            [1, 4, 7]
        ]);
    }
}