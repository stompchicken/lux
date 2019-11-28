#include "vector3.hpp"

#include <iostream>

inline const Vector3 Vector3::zero {0.0, 0.0, 0.0};

std::ostream& operator<<(std::ostream& out, const Vector3& v) {
    out << "{x=" << v.x;
    out << ", y=" << v.y;
    out << ", z=" << v.z;
    out << ", w=" << v.w;
    out << "}";
    return out;
}

bool operator==(const Vector3& lhs, const Vector3& rhs) {
    return lhs.x == rhs.x && lhs.y == rhs.y && lhs.z == rhs.z && lhs.w == rhs.w;
}
