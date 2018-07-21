use std::ops as ops;
use std::f32;
use std::fmt;
use vector::{Vector};

#[derive(PartialEq, Clone, Copy)]
pub struct Matrix {
    pub values: [f32; 16],
}

impl Matrix {

    pub fn zero() -> Matrix {
        Matrix { values: [
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
        ]}
    }

    pub fn identity() -> Matrix {
        Matrix { values: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]}
    }

    pub fn new(values: [f32; 16]) -> Matrix {
        Matrix { values: values}
    }

    fn add(lhs: Matrix, rhs: Matrix) -> Matrix {
        Matrix { values: [
            lhs[0] + rhs[0], lhs[1] + rhs[1], lhs[2] + rhs[2], lhs[3] + rhs[3],
            lhs[4] + rhs[4], lhs[5] + rhs[5], lhs[6] + rhs[6], lhs[7] + rhs[7],
            lhs[8] + rhs[8], lhs[9] + rhs[9], lhs[10] + rhs[10], lhs[11] + rhs[11],
            lhs[12] + rhs[12], lhs[13] + rhs[13], lhs[14] + rhs[14], lhs[15] + rhs[15]
        ]}
    }

    fn sub(lhs: Matrix, rhs: Matrix) -> Matrix {
        Matrix { values: [
            lhs[0] - rhs[0], lhs[1] - rhs[1], lhs[2] - rhs[2], lhs[3] - rhs[3],
            lhs[4] - rhs[4], lhs[5] - rhs[5], lhs[6] - rhs[6], lhs[7] - rhs[7],
            lhs[8] - rhs[8], lhs[9] - rhs[9], lhs[10] - rhs[10], lhs[11] - rhs[11],
            lhs[12] - rhs[12], lhs[13] - rhs[13], lhs[14] - rhs[14], lhs[15] - rhs[15]
        ]}
    }

    fn mat_mul(lhs: Matrix, rhs: Matrix) -> Matrix {
        Matrix { values: [
            lhs[0]*rhs[0] + lhs[1]*rhs[4] + lhs[2]*rhs[8] + lhs[3]*rhs[12],
            lhs[0]*rhs[1] + lhs[1]*rhs[5] + lhs[2]*rhs[9] + lhs[3]*rhs[13],
            lhs[0]*rhs[2] + lhs[1]*rhs[6] + lhs[2]*rhs[10] + lhs[3]*rhs[14],
            lhs[0]*rhs[3] + lhs[1]*rhs[7] + lhs[2]*rhs[11] + lhs[3]*rhs[15],

            lhs[4]*rhs[0] + lhs[5]*rhs[4] + lhs[6]*rhs[8] + lhs[7]*rhs[12],
            lhs[4]*rhs[1] + lhs[5]*rhs[5] + lhs[6]*rhs[9] + lhs[7]*rhs[13],
            lhs[4]*rhs[2] + lhs[5]*rhs[6] + lhs[6]*rhs[10] + lhs[7]*rhs[14],
            lhs[4]*rhs[3] + lhs[5]*rhs[7] + lhs[6]*rhs[11] + lhs[7]*rhs[15],

            lhs[8]*rhs[0] + lhs[9]*rhs[4] + lhs[10]*rhs[8] + lhs[11]*rhs[12],
            lhs[8]*rhs[1] + lhs[9]*rhs[5] + lhs[10]*rhs[9] + lhs[11]*rhs[13],
            lhs[8]*rhs[2] + lhs[9]*rhs[6] + lhs[10]*rhs[10] + lhs[11]*rhs[14],
            lhs[8]*rhs[3] + lhs[9]*rhs[7] + lhs[10]*rhs[11] + lhs[11]*rhs[15],

            lhs[12]*rhs[0] + lhs[13]*rhs[4] + lhs[14]*rhs[8] + lhs[15]*rhs[12],
            lhs[12]*rhs[1] + lhs[13]*rhs[5] + lhs[14]*rhs[9] + lhs[15]*rhs[13],
            lhs[12]*rhs[2] + lhs[13]*rhs[6] + lhs[14]*rhs[10] + lhs[15]*rhs[14],
            lhs[12]*rhs[3] + lhs[13]*rhs[7] + lhs[14]*rhs[11] + lhs[15]*rhs[15]
       ]}
    }

    fn vec_mul(lhs: Matrix, rhs: Vector) -> Vector {
        Vector {
            x: lhs[0]*rhs[0] + lhs[1]*rhs[1] + lhs[2]*rhs[2] + lhs[3]*rhs[3],
            y: lhs[4]*rhs[0] + lhs[5]*rhs[1] + lhs[6]*rhs[2] + lhs[7]*rhs[3],
            z: lhs[8]*rhs[0] + lhs[9]*rhs[1] + lhs[10]*rhs[2] + lhs[11]*rhs[3],
            w: 1.0
        }
    }

    pub fn scale(s: Vector) -> Matrix {
        Matrix { values: [
            s.x, 0.0, 0.0, 0.0,
            0.0, s.y, 0.0, 0.0,
            0.0, 0.0, s.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]}
    }

    pub fn translation(t: Vector) -> Matrix {
        Matrix { values: [
            1.0, 0.0, 0.0, t.x,
            0.0, 1.0, 0.0, t.y,
            0.0, 0.0, 1.0, t.z,
            0.0, 0.0, 0.0, 1.0,
        ]}
    }

    pub fn rotation_x(angle: f32) -> Matrix {
        let sin = angle.sin();
        let cos = angle.cos();

        Matrix { values: [
            1.0, 0.0, 0.0, 0.0,
            0.0, cos, sin, 0.0,
            0.0, -sin, cos, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]}
    }

    pub fn rotation_y(angle: f32) -> Matrix {
        let sin = angle.sin();
        let cos = angle.cos();

        Matrix { values: [
            cos, 0.0, -sin, 0.0,
            0.0, 1.0, 0.0, 0.0,
            sin, 0.0, cos, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]}
    }

    pub fn rotation_z(angle: f32) -> Matrix {
        let sin = angle.sin();
        let cos = angle.cos();

        Matrix { values: [
            cos, sin, 0.0, 0.0,
            -sin, cos, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]}
    }

}

impl ops::Index<usize> for Matrix {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 { return &self.values[i]; }
}

impl ops::Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, other: Matrix) -> Matrix { Matrix::add(self, other) }
}

impl ops::Sub<Matrix> for Matrix {
    type Output = Matrix;
    fn sub(self, other: Matrix) -> Matrix { Matrix::sub(self, other) }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Matrix { Matrix::mat_mul(self, other) }
}

impl ops::Mul<Vector> for Matrix {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector { Matrix::vec_mul(self, other) }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {:.3} {:.3} {:.3} {:.3} ]\n[ {:.3} {:.3} {:.3} {:.3} ]\n[ {:.3} {:.3} {:.3} {:.3} ]\n[ {:.3} {:.3} {:.3} {:.3} ]\n",
               self.values[0], self.values[1], self.values[2], self.values[3],
               self.values[4], self.values[5], self.values[6], self.values[7],
               self.values[8], self.values[9], self.values[10], self.values[11],
               self.values[12], self.values[13], self.values[14], self.values[15])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat4_id() {
        assert!(Matrix::identity() * Matrix::identity() == Matrix::identity());
        assert!(Matrix::identity() * Vector::identity() == Vector::identity());
    }

    #[test]
    fn test_mat4_add_sub() {
        assert!(Matrix::identity() + Matrix::identity() == Matrix::new([2.0, 0.0, 0.0, 0.0,
                                                                        0.0, 2.0, 0.0, 0.0,
                                                                        0.0, 0.0, 2.0, 0.0,
                                                                        0.0, 0.0, 0.0, 2.0]));
        assert!(Matrix::identity() - Matrix::identity() == Matrix::zero());
    }

    #[test]
    fn test_mat4_mul() {
        let a = Matrix::new([1.0, 2.0, 3.0, 4.0,
                             1.0, 2.0, 3.0, 4.0,
                             1.0, 2.0, 3.0, 4.0,
                             1.0, 2.0, 3.0, 4.0]);
        let b = Matrix::new([1.0, 1.0, 1.0, 1.0,
                             2.0, 2.0, 2.0, 2.0,
                             3.0, 3.0, 3.0, 3.0,
                             4.0, 4.0, 4.0, 4.0]);
        let c = Matrix::new([30.0, 30.0, 30.0, 30.0,
                             30.0, 30.0, 30.0, 30.0,
                             30.0, 30.0, 30.0, 30.0,
                             30.0, 30.0, 30.0, 30.0]);
        assert!(a * b == c);
    }

    #[test]
    fn test_mat4_scale() {
        let a = Matrix::scale(Vector::new(2.0, 0.5, 1.5));
        let b = Vector::new(1.0, 2.0, 3.0);
        let c = Vector::new(2.0, 1.0, 4.5);
        assert!(a * b == c);
    }

    #[test]
    fn test_mat4_translation() {
        let a = Matrix::translation(Vector::new(1.0, 1.0, 1.0));
        let b = Vector::new(1.0, 2.0, 3.0);
        let c = Vector::new(2.0, 3.0, 4.0);
        assert!(a * b == c);
    }

    #[test]
    fn test_mat4_rotation_x() {
        let theta = 90.0 as f32;
        let a = Matrix::rotation_x(theta.to_radians());
        let v = Vector::new(1.0, 1.0, 0.0);

        assert!(Vector::approx_eq(a*v, Vector::new(1.0, 0.0, -1.0)));
        assert!(Vector::approx_eq(a*a*v, Vector::new(1.0, -1.0, 0.0)));
        assert!(Vector::approx_eq(a*a*a*v, Vector::new(1.0, 0.0, 1.0)));
        assert!(Vector::approx_eq(a*a*a*a*v, Vector::new(1.0, 1.0, 0.0)));
    }

    #[test]
    fn test_mat4_rotation_y() {
        let theta = 90.0 as f32;
        let a = Matrix::rotation_y(theta.to_radians());
        let v = Vector::new(1.0, 1.0, 0.0);

        assert!(Vector::approx_eq(a*v, Vector::new(0.0, 1.0, 1.0)));
        assert!(Vector::approx_eq(a*a*v, Vector::new(-1.0, 1.0, 0.0)));
        assert!(Vector::approx_eq(a*a*a*v, Vector::new(0.0, 1.0, -1.0)));
        assert!(Vector::approx_eq(a*a*a*a*v, Vector::new(1.0, 1.0, 0.0)));
    }

    #[test]
    fn test_mat4_rotation_z() {
        let theta = 90.0 as f32;
        let a = Matrix::rotation_z(theta.to_radians());
        let v = Vector::new(1.0, 0.0, 1.0);

        assert!(Vector::approx_eq(a*v, Vector::new(0.0, -1.0, 1.0)));
        assert!(Vector::approx_eq(a*a*v, Vector::new(-1.0, 0.0, 1.0)));
        assert!(Vector::approx_eq(a*a*a*v, Vector::new(0.0, 1.0, 1.0)));
        assert!(Vector::approx_eq(a*a*a*a*v, Vector::new(1.0, 0.0, 1.0)));
    }
}
