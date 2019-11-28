#include "light.hpp"

std::ostream& operator<<(std::ostream& out, const Spectrum& s) {
    out << "{r=" << s.r;
    out << ", g=" << s.g;
    out << ", b=" << s.b;
    out << "}";
    return out;
}

Vector3 uniformSphere() {
    Vector3 v { 1.0, 1.0, 1.0 };
    // let mut rng = thread_rng();
    // while p.squared_length() >= 1.0 {
    //     p = 2.0 * Vector::new(rng.gen(),
    //                         rng.gen(),
    //                         rng.gen()) - Vector::new(1.0, 1.0, 1.0);
    // }
    return v;
}


std::ostream& operator<<(std::ostream& out, const Hit& hit) {
    if (auto reflection = std::get_if<Reflection>(&hit.type)) {
        out << "{Hit: Reflection " << reflection->reflected.spectrum << "}";
    } else if (auto emission = std::get_if<Emission>(&hit.type)) {
        out << "{Hit: Emission " << emission->emitted << "}";
    }
    return out;
}
