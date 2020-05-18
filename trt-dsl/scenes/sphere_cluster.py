from trt import render
from trt.shape import sphere, bvh_node
from trt.material import matte, metallic, dielectric, diffuse_color

from random import random as rand, seed, choices

def random_color():
    return (rand() * rand(), rand() * rand(), rand() * rand())

def spheres():
    material_choices = (
        (lambda: matte(random_color()),                   0.5),
        (lambda: metallic(random_color(), 0.25 * rand()), 0.4),
        (lambda: dielectric(1.0 * rand()),                0.1)
    )

    materials = iter(choices(*zip(*material_choices), k=400))

    for dx in range(-10, 10):
        for dz in range(-10, 10):
            center = (dx + 0.9 * rand(), 0.2, dz + 0.9 * rand())

            material = next(materials)()

            yield sphere(center, 0.2, material)

    yield from [
        sphere((0, 1, 0), 1, dielectric(1.5)),
        sphere((-4, 1, 0), 1, matte((.4, .2, .1))),
        sphere((4, 1, 0), 1, metallic((.7, .6, .5))),
    ]

def scene():
    seed(0xDEADBEEF)

    ground = sphere((0, -1000, 0), 1000, metallic((0.5, 0.5, 0.5)))

    return [bvh_node(spheres()), ground]

def config(spx):
    return {
        'width': 600,
        'height': 450,
        'samples_per_px': spx,
        'rays_per_sample': 50,
        'ambiant_color': (0.5, 0.7, 0.9),
        'camera': {
            'look_at': (0, 0, 0),
            'look_from': (10, 2, 4),
        },
    }

render(scene(), **config(25))
