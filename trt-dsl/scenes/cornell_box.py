from trt.material import matte, metallic, dielectric, diffuse_color, image
from trt.shape import sphere, rect
from trt import render

def scene():
    red = (0.9, 0.3, 0.3)
    green = (0.3, 0.9, 0.3)
    blue = (0.3, 0.3, 0.9)
    white = (0.7, 0.7, 0.7)

    spheres = [
        sphere(
            center=(150+ i * 150, 150 + i * 100, 150 + i * 100),
            radius=80,
            material=metallic(color),
        )
        for i, color in enumerate((red, green, blue))
    ]

    cornell_box = [
        # ceiling
        rect(x=(0, 600), z=(-1000, 600), y=600, material=matte(white)).flip_normals(),
        # light
        rect(x=(100, 500), z=(100, 400), y=599, material=diffuse_color((7, 7, 7))),
        # floor
        rect(x=(0, 600), z=(-1000, 600), y=0, material=matte(white)),
        # left
        rect(y=(0, 600), z=(-1000, 600), x=600, material=matte(red)).flip_normals(),
        # right
        rect(y=(0, 600), z=(-1000, 600), x=0, material=matte(green)),
        # front
        rect(x=(0, 600), y=(0, 600), z=600, material=matte(white)).flip_normals(),
        # back
        rect(x=(0, 600), y=(0, 600), z=-1000, material=matte(white)),
    ]

    return cornell_box + spheres

config = {
    'width': 400,
    'height': 400,
    'samples_per_px': 50,
    "camera": {
        'look_at': (300, 300, 0),
        'look_from': (300, 300, -800)
    }
}

render(scene(), **config)
