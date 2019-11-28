#include <iostream>
#include <cassert>

#include "vector3.hpp"
#include "matrix3.hpp"
#include "light.hpp"
#include "geometry.hpp"
#include "object.hpp"
#include "material.hpp"
#include "renderer.hpp"

int main() {
    Vector3 v1 {1.0, 1.0, 1.0};
    Vector3 v2 {0.5, 2.0, 0.5};

    assert(Vector3::zero == Vector3(0.0, 0.0, 0.0));
    assert(v1 + v2 == Vector3(1.5, 3.0, 1.5));
    assert(v1 - v2 == Vector3(0.5, -1.0, 0.5));

    assert(v1 * 0.5 == Vector3(0.5, 0.5, 0.5));
    assert(v2 / 0.5 == Vector3(1.0, 4.0, 1.0));

    v1 += v2;
    assert(v1 == Vector3(1.5, 3.0, 1.5));
    v1 -= v2;
    assert(v1 == Vector3(1.0, 1.0, 1.0));

    assert(Vector3::dot(v1, v2) == 3);

    Stats stats;
    Config config;
    Renderer renderer;
    renderer.render(config, stats);

    std::cout << stats << std::endl;

    std::cout << "Praise the sun!" << std::endl;

    return 0;
}
