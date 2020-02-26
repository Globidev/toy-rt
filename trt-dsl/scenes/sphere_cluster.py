from trt import render
from trt.shape import sphere, bvh_node
from trt.material import matte, metallic, dielectric

from random import random as rand, seed, choice

def random_color():
    return (rand() * rand(), rand() * rand(), rand() * rand())

def spheres():
    for a in range(-11, 11):
        for b in range(-11, 11):
            center = (a + 0.9 * rand(), 0.2, b + 0.9 * rand())

            material = choice([
                matte(random_color()),
                metallic(random_color(), 0.25 * rand()),
                dielectric(1.0 * rand())
            ])

            yield sphere(center, 0.2, material)

    yield from [
        sphere((0, 1, 0), 1, dielectric(1.5)),
        sphere((-4, 1, 0), 1, matte((.4, .2, .1))),
        sphere((4, 1, 0), 1, metallic((.7, .6, .5))),
    ]

def scene():
    ground = sphere((0, -200, 0), 200, metallic((0.5, 0.5, 0.5)))

    return [bvh_node(spheres()), bvh_node([ground])]

seed(0xDEADBEEF)

config = {
    'width': 600,
    'height': 450,
    'ambiant_color': (0.5, 0.7, 0.9),
    'camera': {
        'look_at': (0, 0, 0),
        'look_from': (10, 2, 4),
    },
}

render(scene(), **config)
