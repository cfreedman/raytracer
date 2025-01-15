# A Rust Raytracer

An implementation of a raytracer from first principles of graphics programming based upon the series _Raytracing in One Weekend_. Various graphical features are included such as

#### Geometries

- Spherical intersection
- Quad intersection

#### Textures

- Texture mapping
- Solid color
- Checkered

#### Materials

- Lambertian
- Dielectrics
- Metals

- Emissive Lights
- Volumes

Standard acceleration techniques for the raytracing algorithm are available like **bounded volume hierarchy** for sorting the objects in scene according to the bounding boxes and organizing them into a tree-like structure for fastest ray-intersection calculation.

See some example outputs below!

### Cornell Box

![cornell-box](image.ppm)
