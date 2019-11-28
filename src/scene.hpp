#ifndef SCENE_HPP
#define SCENE_HPP

#include <vector>
#include <memory>
#include "object.hpp"

#include <iostream>

class Scene {
public:
    void addObject(const Object& object) {
        objects.emplace_back(object);
    }

    bool propagateRay(const Ray& ray, float t_min, float t_max, Hit& hit) const {
        float closest = t_max;
        Hit tempHit;

        for (const auto& obj : objects) {
            if(obj.propagateRay(ray, t_min, closest, tempHit)) {
                closest = tempHit.time;
                hit = tempHit;
            };
        }

        return closest != t_max;
    }


protected:
    std::vector<Object> objects;

};

#endif
