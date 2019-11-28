#ifndef LIGHT_HPP
#define LIGHT_HPP

#include <variant>
#include <iostream>

#include "vector3.hpp"

struct Spectrum {
    float r = 0.0;
    float g = 0.0;
    float b = 0.0;

    inline Spectrum operator+(const Spectrum& s) const {
        return Spectrum {r + s.r, g + s.g, b + s.b};
    }

    inline Spectrum& operator+=(const Spectrum& s) {
        r += s.r;
        g += s.g;
        b += s.b;
        return *this;
    }

    inline Spectrum operator-(const Spectrum& s) const {
        return Spectrum {r - s.r, g - s.g, b - s.b};
    }

    inline Spectrum& operator-=(const Spectrum& s) {
        r -= s.r;
        g -= s.g;
        b -= s.b;
        return *this;
    }

    inline Spectrum operator*(float k) const {
        return Spectrum {r*k, g*k, b*k};
    }

    inline Spectrum& operator*=(float k) {
        r *= k;
        g *= k;
        b *= k;
        return *this;
    }

   inline Spectrum operator/(float k) const {
        return Spectrum {r/k, g/k, b/k};
    }

    inline Spectrum& operator/=(float k) {
        r /= k;
        g /= k;
        b /= k;
        return *this;
    }

    inline static Spectrum combine(const Spectrum& lhs, const Spectrum& rhs) {
        return Spectrum { lhs.r * rhs.r, lhs.g * rhs.g, lhs.b * rhs.b};
    }

};

std::ostream& operator<<(std::ostream& out, const Spectrum& s);

namespace Colour {
    inline static constexpr Spectrum black = {0.0, 0.0, 0.0};
    inline static constexpr Spectrum white = {1.0, 1.0, 1.0};
}

struct Ray {
    Vector3 origin;
    Vector3 direction;
    Spectrum spectrum;

    Vector3 pointAt(float t) const {
        return origin + (direction * t);
    }
};

struct Emission {
    Spectrum emitted;
};

struct Reflection {
    Ray reflected;
};

struct Hit {
    std::variant<Emission, Reflection> type;
    float time = 0;
};

Vector3 uniformSphere();


std::ostream& operator<<(std::ostream& out, const Hit& hit);

#endif
