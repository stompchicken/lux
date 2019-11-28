#ifndef MATERIAL_HPP
#define MATERIAL_HPP

#include "light.hpp"
#include "geometry.hpp"

class Material {
public:
    virtual ~Material() = default;
    virtual Hit hit(const Ray& ray, const Incidence& incidence) const = 0;
};

class NormalSurface : public Material {
public:
    virtual Hit hit(const Ray& ray, const Incidence& incidence) const {
        (void)ray; // unused
        Vector3 norm = incidence.surfaceNormal.normalise();
        Spectrum spectrum = { 0.5f * (norm.x + 1.0f),
                              0.5f * (norm.z + 1.0f),
                              0.5f * (norm.y + 1.0f)};
        return Hit { Emission { spectrum } , incidence.time };
    }
};

class DiffuseEmitter : public Material {
public:
    DiffuseEmitter(const Spectrum& spectrum) :
        spectrum(spectrum) {
    }

    virtual Hit hit(const Ray& ray, const Incidence& incidence) const {
        (void)ray; // unused
        return Hit { Emission { spectrum } , incidence.time };
    }

protected:
    Spectrum spectrum = Colour::white;
};

class DiffuseSurface : public Material {
public:
    DiffuseSurface(const Spectrum& spectrum) :
        albedo(spectrum) {
    }

    virtual Hit hit(const Ray& ray, const Incidence& incidence) const {
        Vector3 target = incidence.position + incidence.surfaceNormal + uniformSphere();
        Ray reflected = {
            incidence.position,
            target - incidence.position,
            Spectrum::combine(ray.spectrum, albedo)
        };
        return Hit { Reflection { reflected }, incidence.time };
    }

protected:
    Spectrum albedo = Colour::white;
};

#endif
