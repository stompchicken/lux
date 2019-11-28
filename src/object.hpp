#ifndef OBJECT_HPP
#define OBJECT_HPP

#include <memory>

#include "matrix3.hpp"
#include "light.hpp"
#include "geometry.hpp"
#include "material.hpp"

struct Object {
    const Geometry& geometry;
    const Material& material;
    //Matrix3 transform;

    bool propagateRay(const Ray& ray, float t_min, float t_max, Hit& hit) const {
        Incidence incidence;
        if (this->geometry.testCollision(ray, t_min, t_max, incidence)) {
            hit = this->material.hit(ray, incidence);
            return true;
        } else {
            return false;
        }

    }
};

#endif
