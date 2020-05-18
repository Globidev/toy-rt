from trt import render
from trt.shape import sphere, bvh_node, hitbox
from trt.material import image, metallic

from random import seed, random as rand

FERRIS_SWEAT = image("https://glo.bi/static/ferris_sweat.png")
FERRIS_UNSAFE = image("https://glo.bi/static/ferris_unsafe.png")
GOPHER_PEEK = image('https://glo.bi/static/gopher_peek.png')
GOPHER = image('https://glo.bi/static/gopher.png')

def random_position_in_cube(size):
    return (rand() * size, rand() * size, rand() * size)

def ground():
    tile_size = 100
    light_green = (0.48, 0.83, 0.53)

    for dx in range(15):
        for dz in range(40):
            x0 = (-500 + dx * tile_size)
            z0 = (-2000 + dz * tile_size)
            mat = metallic(light_green) if rand() < 0.9 else FERRIS_UNSAFE
            yield hitbox((x0, 0, z0), (x0 + tile_size, 10, z0 + tile_size), mat)

def scene():
    seed(0xDEADBEEF)

    foam_ferris = bvh_node(
        sphere((0, 0, 0), 50, FERRIS_SWEAT)
            .rotate_y(rand() * 180)
            .translate(random_position_in_cube(800))
        for _count in range(500)
    )

    foam_gopher = bvh_node(
        sphere((0, 0, 0), 150, GOPHER)
            .rotate_y(rand() * 100)
            .rotate_x(-50)
            .translate(random_position_in_cube(1200))
        for _count in range(150)
    )

    return [
        bvh_node(ground()),

        foam_ferris.translate((200, 200, 2000)),
        foam_gopher.translate((-1400, 800, 3000)),

        # Gopher cube
        hitbox((0, 0, 0), (4000, 5000, 5000), GOPHER_PEEK)
            .rotate_y(-50)
            .rotate_x(50)
            .translate((500, 2000, 5000))
    ]

def config(spx):
    white_ish = (0.7, 0.8, 0.9)

    return {
        "samples_per_px": spx,
        "ambiant_color": white_ish,
        "camera": {
            "look_at": (300, 600, 0),
            "look_from": (400, 600, -1000),
        }
    }

render(scene(), **config(25))
