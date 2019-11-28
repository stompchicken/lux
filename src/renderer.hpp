#ifndef RENDERER_HPP
#define RENDERER_HPP

#include <iostream>

#include "light.hpp"
#include "scene.hpp"

struct Stats {
    int rays = 0;
    int emissions = 0;
    int reflections = 0;
    int noHits = 0;
    int maxDepth = 0;
};

struct Config {
    int maxBounces = 1;
    Spectrum background = Spectrum { 0.5, 0.5, 0.5 };
    int width = 128;
    int height = 128;
    int numRays = 1;
    bool randomiseRays = false;
};

std::ostream& operator<<(std::ostream& out, const Stats& stats);

class Renderer {
public:
    Scene createScene() const;
    void render(const Config& config, Stats& stats) const;
    Spectrum traceRay(const Ray& ray, const Scene& scene, const Config& config, int depth, Stats& stats) const;

protected:
    Stats stats;
};

#endif
