#include "image.hpp"

#include <fstream>
#include <iostream>

bool Image::writePPM(const std::string& path) const {
    std::ofstream ofile(path, std::ios::binary);

    ofile << "P6 " << width << " " << height << " 255" << std::endl;

    for (const auto& pixel : data) {
        ofile << pixel.r << pixel.g << pixel.b;
    }

    return true;
}
