#include "matrix3.hpp"

#include <iterator>
#include <iostream>

inline const Matrix3 Matrix3::zero {0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0};
inline const Matrix3 Matrix3::identity {0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0};

std::ostream& operator<<(std::ostream& out, const Matrix3& m) {
    copy(m.values.cbegin(), m.values.cend(), std::ostream_iterator<float>(out, " "));
    return out;
}

bool operator==(const Matrix3& lhs, const Matrix3& rhs) {
    for(int i=0; i<16; i++) {
        if (lhs[i] != rhs[i]) {
            return false;
        }
    }

    return true;
}
