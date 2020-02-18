from trt import Sphere, Rect, Scene, Camera, Material, BVHNode, HitBox, rand

FERRIS_SWEAT = Material.image("https://cdn.discordapp.com/emojis/448264617991602186.png?v=1")
FERRIS_UNSAFE = Material.image("https://cdn.discordapp.com/emojis/358652666265731072.png?v=1")

def ground():
    for dx in range(20):
        for dz in range(40):
            w = 100
            x0 = (-1000 + dx * w)
            z0 = (-2000 + dz * w)
            x1 = x0 + w
            z1 = z0 + w
            choose_mat = rand()
            if choose_mat < 0.8:
                mat = Material.metallic((0.48, 0.83, 0.53))
            else:
                mat = FERRIS_UNSAFE
            yield HitBox((x0, 0, z0), (x1, 0.1, z1), mat)

def foam():
    for _ in range(1000):
        center = (rand() * 800, rand() * 800, rand() * 800)
        yield Sphere(center, 50, FERRIS_SWEAT).rotate_y(360 * rand())


def scene():
    ground = BVHNode(list(ground()))

    foam = BVHNode(list(foam())).translate((100, 200, 2000))

    light = Rect(x=(150, 450), z=(200, 400), y=550, material=Material.diffuse_color((8,8,8)))

    return Scene(**{
        "world": [ground, foam],
        "width": 400,
        "height": 400,
        "samples_per_px": 10,
        "rays_per_sample": 25,
        "ambiant_color": (0.98, 0.95, 0.63),
        "camera": Camera(
            look_at=(278, 278, 0),
            look_from=(378, 278, -1000),
        )
    })
