from trt import render
from trt.shape import sphere, rect, bvh_node, hitbox
from trt.material import image, metallic

from random import seed, random as rand

FERRIS_SWEAT = image("https://cdn.discordapp.com/emojis/448264617991602186.png?v=1")
FERRIS_UNSAFE = image("https://cdn.discordapp.com/emojis/358652666265731072.png?v=1")
GOPHER_PEEK = image('https://cors-anywhere.herokuapp.com/https://cdn.discordapp.com/attachments/178321904904568832/679154014083874816/golang-gopher-hello.png')
GOPHER = image('https://cors-anywhere.herokuapp.com/https://cdn.discordapp.com/attachments/178321904904568832/679153876473217054/ap550x55012x161transparentt.png')

def random_color():
    return (rand() * rand(), rand() * rand(), rand() * rand())

def ground():
    w = 100

    for dx in range(20):
        for dz in range(40):
            x0 = (-1000 + dx * w)
            z0 = (-2000 + dz * w)
            mat = metallic((0.48, 0.83, 0.53)) if rand() < 0.9 else FERRIS_UNSAFE
            yield hitbox((x0, 0, z0), (x0 + w, 10, z0 + w), mat)

def foam_ferris():
    for _ in range(1000):
        center = (0, 0, 0)
        d = (rand() * 800, rand() * 800, rand() * 800)
        yield sphere(center, 50, FERRIS_SWEAT).rotate_y(180 * rand()).translate(d)

def foam_gopher():
    for _ in range(250):
        center = (0, 0, 0)
        d = (rand() * 1200, rand() * 1200, rand() * 1200)
        yield sphere(center, 150, GOPHER).rotate_y(rand() * 100).rotate_x(-50).translate(d)

def scene():
    ground = bvh_node(ground())

    foam_ferris = bvh_node(foam_ferris()).translate((100, 200, 2000))
    foam_gopher = bvh_node(foam_gopher()).translate((-1500, 800, 3000))

    back_wall = rect(x=(-2000, 2000), y=(-2000, 3000), z=5000, material=GOPHER_PEEK)

    return [ground, foam_ferris, foam_gopher, back_wall]

seed(0xDEADBEEF)

config = {
    "samples_per_px": 25,
    "ambiant_color": (0.7, 0.7, 0.9),
    "camera": {
        "look_at": (278, 600, 0),
        "look_from": (378, 600, -1000),
    }
}

render(scene(), **config)
