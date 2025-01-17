# A Rust Raytracer

An implementation of a raytracer from first principles of graphics programming based upon the series _Raytracing in One Weekend_. Various graphical features are included such as

#### Geometries

[x] Spherical intersection
[x] Quad intersection
[x] Instancing translation and rotation

#### Textures

[x] Texture mapping
[x] Solid color
[x] Checkered

#### Materials

[x] Lambertian
[x] Dielectrics
[x] Metals

[x] Emissive Lights
[x] Volumes

Standard acceleration techniques for the raytracing algorithm are available like **bounded volume hierarchy** for sorting the objects in scene according to the bounding boxes and organizing them into a tree-like structure for fastest ray-intersection calculation.

See some example outputs below!

### Cornell Box

Motion blur spheres with a variety of materials

[!Motion blur bouncing balls](/output/bouncing_balls.png)

Metallic Materials

[!Metallic materials](/output/metal.png)

Checkered Texturess

[!Checkered Textures](/output/checkered.png)
