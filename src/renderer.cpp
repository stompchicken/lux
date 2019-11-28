#include "renderer.hpp"

#include <cassert>
#include <random>

#include "light.hpp"
#include "object.hpp"
#include "scene.hpp"
#include "camera.hpp"
#include "image.hpp"

    int emissions = 0;
    int reflections = 0;
    int noHits = 0;
    int maxDepth = 0;

std::ostream& operator<<(std::ostream& out, const Stats& stats) {
    out << "Rays: " << stats.rays << std::endl;
    out << "Emitted: " << stats.emissions << std::endl;
    out << "Reflected: " << stats.reflections << std::endl;
    out << "NoHits: " << stats.noHits << std::endl;
    out << "MaxDepths: " << stats.maxDepth << std::endl;
    return out;
}

Spectrum Renderer::traceRay(const Ray& ray, const Scene& scene, const Config& config, int depth, Stats& stats) const {
    if (depth >= config.maxBounces) {
        stats.maxDepth += 1;
        return Spectrum::combine(ray.spectrum, config.background);
    }

    Hit hit;
    if (scene.propagateRay(ray, 0.001, 1000.0, hit)) {
        if (auto reflection = std::get_if<Reflection>(&hit.type)) {
            stats.reflections += 1;
            return Spectrum::combine(ray.spectrum, traceRay(reflection->reflected, scene, config, depth + 1, stats));
        } else if (auto emission = std::get_if<Emission>(&hit.type)) {
            stats.emissions += 1;
            return emission->emitted;
        } else {
            assert(false);
            return Spectrum(); // unreachable
        }
    } else {
        stats.noHits += 1;
        return Spectrum::combine(ray.spectrum, config.background);
    }
}

std::vector<Ray> generateRays(const Camera& camera, const Config& config, std::default_random_engine& gen, int x, int y, int width, int height, int numRays) {
    std::vector<Ray> rays;
    float rangeMin, rangeMax;
    if (config.randomiseRays) {
        numRays = 1;
        rangeMin = 0.0;
        rangeMax = 0.0;
    } else {
        rangeMin = -0.5;
        rangeMax = 0.5;
    }

    rays.reserve(numRays);
    std::uniform_real_distribution<float> rand(rangeMin, rangeMax);

    for (int n=0; n<numRays; n++) {
        float px = x + rand(gen);
        float py = y + rand(gen);

        float u = (px / width) - 0.5;
        float v = (py / height) - 0.5;

        rays.push_back(camera.createRay(u, v));
    }

    return rays;
}

void Renderer::render(const Config& config, Stats& stats) const {

    Camera camera(
        Vector3(0.0, 0.0, -5.0),
        Vector3(0.0, 0.0, 0.0),
        Vector3(0.0, 1.0, 0.0),
        90.0,
        1.0);


    Scene scene;

    DiffuseSurface triangleMaterial1 {
        Spectrum { 1.0, 0.0, 0.0 }
    };
    Triangle triangleGeometry1 {
        Vector3 { -1.0,  1.0,  0.0 },
        Vector3 { -1.0, -1.0,  0.0 },
        Vector3 { -1.0, -1.0, -1.0 }
    };
    Object triangle1 {
        triangleGeometry1,
        triangleMaterial1
    };

    scene.addObject(triangle1);

    Triangle triangleGeometry2 {
        Vector3 { -1.0,  1.0, -1.0 },
        Vector3 { -1.0,  1.0,  0.0 },
        Vector3 { -1.0, -1.0, -1.0 }
    };
    Object triangle2 {
        triangleGeometry2,
        triangleMaterial1
    };

    scene.addObject(triangle2);

    DiffuseSurface triangleMaterial2 {
        Spectrum { 0.0, 1.0, 0.0 }
    };
    Triangle triangleGeometry3 {
        Vector3 { 1.0,  1.0,  0.0 },
        Vector3 { 1.0, -1.0,  0.0 },
        Vector3 { 1.0, -1.0, -1.0 }
    };
    Object triangle3 {
        triangleGeometry3,
        triangleMaterial2
    };

    scene.addObject(triangle3);

    Triangle triangleGeometry4 {
        Vector3 { 1.0,  1.0, -1.0 },
        Vector3 { 1.0,  1.0,  0.0 },
        Vector3 { 1.0, -1.0, -1.0 }
    };
    Object triangle4 {
        triangleGeometry4,
        triangleMaterial2
    };

    scene.addObject(triangle4);



    
    Image image(config.width, config.height);

    std::random_device rd;
    std::default_random_engine gen(rd());

    for(int x=0; x<config.width; x++) {
        for(int y=0; y<config.height; y++) {
            std::vector<Ray> rays = generateRays(camera, config, gen, x, y, config.width, config.height, config.numRays);

            Spectrum spectrum = Colour::black;
            for (const auto& ray : rays) {
                stats.rays += 1;
                spectrum += traceRay(ray, scene, config, 0, stats);
            }

            spectrum /= static_cast<float>(config.numRays);

            Pixel pixel {
                static_cast<uint8_t>(spectrum.r * 255),
                    static_cast<uint8_t>(spectrum.g * 255),
                    static_cast<uint8_t>(spectrum.b * 255)
                    };

            image.setPixel(x, y, pixel);
        }
    }

    image.writePPM("test.ppm");
}
