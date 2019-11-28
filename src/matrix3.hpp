#ifndef MATRIX3_HPP
#define MATRIX3_HPP

#include <iostream>
#include <cmath>
#include <array>
#include <algorithm>

#include "vector3.hpp"

struct Matrix3 {
    std::array<float, 16> values;

    Matrix3() = default;

    inline Matrix3(float v1, float v2, float v3, float v4, float v5, float v6, float v7, float v8, float v9, float v10, float v11, float v12, float v13, float v14, float v15, float v16) {
        values[0] = v1;
        values[1] = v2;
        values[2] = v3;
        values[3] = v4;
        values[4] = v5;
        values[5] = v6;
        values[6] = v7;
        values[7] = v8;
        values[8] = v9;
        values[9] = v10;
        values[10] = v11;
        values[11] = v12;
        values[12] = v13;
        values[13] = v14;
        values[14] = v15;
        values[15] = v16;
    }

    float operator[](std::size_t i) const {
         return values[i];
    }

    inline Matrix3 operator+(const Matrix3& m) const {
        return Matrix3({
            values[0] + m[0], values[1] + m[1], values[2] + m[2], values[3] + m[3],
            values[4] + m[4], values[5] + m[5], values[6] + m[6], values[7] + m[7],
            values[8] + m[8], values[9] + m[9], values[10] + m[10], values[11] + m[11],
            values[12] + m[12], values[13] + m[13], values[14] + m[14], values[15] + m[15]});
    }

    inline Matrix3& operator+=(const Matrix3& m) {
        for(int i=0; i<16; i++) {
            values[i] += m[i];
        }
        return *this;
    }

    inline Matrix3 operator-(const Matrix3& m) const {
        return Matrix3({
            values[0] - m[0], values[1] - m[1], values[2] - m[2], values[3] - m[3],
            values[4] - m[4], values[5] - m[5], values[6] - m[6], values[7] - m[7],
            values[8] - m[8], values[9] - m[9], values[10] - m[10], values[11] - m[11],
            values[12] - m[12], values[13] - m[13], values[14] - m[14], values[15] - m[15]
        });
    }

    inline Matrix3& operator-=(const Matrix3& m) {
        for(int i=0; i<16; i++) {
            values[i] -= m[i];
        }
        return *this;
    }

    inline Matrix3 operator*(float k) const {
        return Matrix3({
            values[0] * k, values[1] * k, values[2] * k, values[3] * k,
            values[4] * k, values[5] * k, values[6] * k, values[7] * k,
            values[8] * k, values[9] * k, values[10] * k, values[11] * k,
            values[12] * k, values[13] * k, values[14] * k, values[15] * k
        });
    }

    inline Matrix3& operator*=(float k) {
        for(int i=0; i<16; i++) {
            values[i] *= k;
        }
        return *this;
    }

    inline Matrix3 operator/(float k) const {
        return Matrix3({
            values[0] / k, values[1] / k, values[2] / k, values[3] / k,
            values[4] / k, values[5] / k, values[6] / k, values[7] / k,
            values[8] / k, values[9] / k, values[10] / k, values[11] / k,
            values[12] / k, values[13] / k, values[14] / k, values[15] / k
        });
    }

    inline Matrix3& operator/=(float k) {
        for(int i=0; i<16; i++) {
            values[i] /= k;
        }
        return *this;
    }

    inline static Matrix3 matMul(const Matrix3& lhs, const Matrix3& rhs) {
        return Matrix3 {
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
        };
    }

    static const Matrix3 zero;
    static const Matrix3 identity;
};

std::ostream& operator<<(std::ostream& out, const Matrix3& v);

bool operator==(const Matrix3& lhs, const Matrix3& rhs);


#endif
