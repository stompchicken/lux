#ifndef IMAGE_HPP
#define IMAGE_HPP

#include <cstdint>
#include <vector>
#include <string>

struct Pixel {
    uint8_t r = 0.0;
    uint8_t g = 0.0;
    uint8_t b = 0.0;
    uint8_t a = 1.0;
};

class Image {
public:
    Image(int width, int height) :
        width(width), height(height) {
        data.resize(width * height);
    }

    void setPixel(int x, int y, Pixel px) {
        // Input is in projected coordinate system
        // Output is in image coordinat system
        data[((width - y - 1) * width) + x] = px;
    }

    bool writePPM(const std::string& path) const;

protected:
    int width;
    int height;
    std::vector<Pixel> data;

};

#endif
