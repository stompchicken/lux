#ifndef VECTOR3_HPP
#define VECTOR3_HPP

#include <iostream>
#include <cmath>

struct Vector3 {
    float x = 0.0;
    float y = 0.0;
    float z = 0.0;
    float w = 1.0;

    inline Vector3() = default;

    inline Vector3(float x, float y, float z) :
        x(x), y(y), z(z), w(1.0) {
    }

    inline Vector3(float x, float y, float z, float w) :
        x(x), y(y), z(z), w(w) {
    }

    inline Vector3 operator+(const Vector3& v) const {
        return Vector3(x + v.x, y + v.y, z + v.z);
    }

    inline Vector3& operator+=(const Vector3& v) {
        x += v.x;
        y += v.y;
        z += v.z;
        return *this;
    }

    inline Vector3 operator-(const Vector3& v) const {
        return Vector3(x - v.x, y - v.y, z - v.z);
    }

    inline Vector3& operator-=(const Vector3& v) {
        x -= v.x;
        y -= v.y;
        z -= v.z;
        return *this;
    }

    inline Vector3 operator*(float k) const {
        return Vector3(x*k, y*k, z*k);
    }

    inline Vector3& operator*=(float k) {
        x *= k;
        y *= k;
        z *= k;
        return *this;
    }

    inline Vector3 operator/(float k) const {
        return Vector3(x/k, y/k, z/k);
    }

    inline Vector3& operator/=(float k) {
        x /= k;
        y /= k;
        z /= k;
        return *this;
    }

    inline static float dot(const Vector3& lhs, const Vector3& rhs) {
        return (lhs.x * rhs.x) + (lhs.y * rhs.y) + (lhs.z * rhs.z);
    }

    inline static Vector3 cross(const Vector3& lhs, const Vector3& rhs) {
        return Vector3((lhs.y * rhs.z) - (lhs.z * rhs.y),
                       (lhs.x * rhs.z) - (lhs.z * rhs.x),
                       (lhs.x * rhs.y) - (lhs.y * rhs.x));
    }

    inline float squared_length() const {
        return (x*x) + (y*y) + (z*z);
    }

    inline float length() const {
        return std::sqrt((x*x) + (y*y) + (z*z));
    }

    inline Vector3 normalise() const {
        float length = this->length();
        return Vector3(x / length, y / length, z / length);
    }

    static const Vector3 zero;
};

std::ostream& operator<<(std::ostream& out, const Vector3& v);

bool operator==(const Vector3& lhs, const Vector3& rhs);


#endif
