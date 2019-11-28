#ifndef GEOM_HPP
#define GEOM_HPP

#include <memory>

#include "vector3.hpp"
#include "light.hpp"

struct Incidence {
    Vector3 position;
    Vector3 surfaceNormal;
    float time;
};

class Geometry {
public:
    virtual ~Geometry() = default;

    auto clone() const {
        return std::unique_ptr<Geometry>(clone_impl());
    }

    virtual bool testCollision(const Ray& ray, float t_min, float t_max, Incidence& result) const = 0;

protected:
    virtual Geometry* clone_impl() const = 0;
};

class Sphere : public Geometry {
public:

    Sphere(const Vector3 position, float radius) :
        position(position), radius(radius) {
    }

    virtual bool testCollision(const Ray& ray, float t_min, float t_max, Incidence& result) const override {
        Vector3 oc = ray.origin - this->position;
        float a = Vector3::dot(ray.direction, ray.direction);
        float b = Vector3::dot(oc, ray.direction);
        float c = Vector3::dot(oc, oc) - this->radius * this->radius;
        float d = b*b - a*c;

        if (d > 0.0) {
            float t1 = (-b - std::sqrt(b*b - a*c)) / a;
            float t2 = (-b + std::sqrt(b*b - a*c)) / a;

            bool hit = false;
            float t = 0.0;
            if ((t1 < t_max) && (t1 > t_min)) {
                hit = true;
                t = t1;
            } else if ((t2 < t_max) && (t2 > t_min)) {
                hit = true;
                t = t2;
            }

            if (hit) {
                result.time = t;
                result.position = ray.pointAt(t);
                result.surfaceNormal = (ray.pointAt(t) - this->position) / this->radius;
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

protected:
    Vector3 position = Vector3::zero;
    float radius = 1.0;

    virtual Sphere* clone_impl() const override { return new Sphere(*this); };
};


class Triangle : public Geometry {
public:

    Triangle(const Vector3& v0, const Vector3& v1, const Vector3& v2) :
        v0(v0), v1(v1), v2(v2), normal(Vector3::cross(v1 - v0, v2 - v0).normalise()) {
    }

    virtual bool testCollision(const Ray& ray, float t_min, float t_max, Incidence& result) const override {
        // Angle between plane and ray
        float theta = Vector3::dot(normal, ray.direction.normalise());

        // Ray and plane are approximately parallel
        if (fabs(theta) < 0.0001) {
            return false;
        }

        float d = Vector3::dot(normal, v0);
        float t = -(Vector3::dot(normal, ray.origin) + d) / theta;
        // Ray/plane intersection point
        Vector3 p = ray.origin + (ray.direction * t);

        // Intersection point is out of bounds
        if (t < t_min || t >= t_max) {
            return false;
        }


        Vector3 c0 = Vector3::cross(v1 - v0, p - v0);
        if (Vector3::dot(normal, c0) < 0) {
            return false;
        }

        Vector3 c1 = Vector3::cross(v2 - v1, p - v1);
        if (Vector3::dot(normal, c1) < 0) {
            return false;
        }

        Vector3 c2 = Vector3::cross(v0 - v2, p - v2);
        if (Vector3::dot(normal, c2) < 0) {
            return false;
        }

        result.position = p;
        result.surfaceNormal = normal;
        result.time = t;

        return true;
    }

protected:
    Vector3 v0, v1, v2;
    Vector3 normal;

    virtual Triangle* clone_impl() const override { return new Triangle(*this); };

};

#endif
