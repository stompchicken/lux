#ifndef CAMERA_HPP
#define CAMERA_HPP

#include <cmath>

#include "vector3.hpp"
#include "light.hpp"

struct Camera {
    Vector3 origin;
    Vector3 direction;
    Vector3 horizontal;
    Vector3 vertical;
    Vector3 lowerLeft;

    Camera(Vector3 lookFrom, Vector3 lookAt, Vector3 up, float fov, float aspectRatio) {
        float theta = (fov * M_PI) / 180.0 ;
        float halfWidth = tan(theta / 2.0);
        float halfHeight = halfWidth / aspectRatio;

        Vector3 z = (lookAt - lookFrom).normalise();
        Vector3 x = Vector3::cross(up, z).normalise();
        Vector3 y = Vector3::cross(x, z).normalise();

        this->origin = lookFrom;
        this->direction = z;

        this->horizontal = x * halfWidth;
        this->vertical = y * halfHeight;
        this->lowerLeft = lookFrom - (x * halfWidth) - (y * halfHeight);
    }

    Ray createRay(float u, float v) const {
        return Ray {
            this->origin,
            this->direction + (this->horizontal * u) + (this->vertical * v),
            Colour::white
        };
    }

};

#endif
