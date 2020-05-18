from trt import render
from trt.shape import sphere
from trt.material import matte, metallic

def scene():
    red = (1, 0, 0)
    green = (0, 1, 0)
    blue = (0, 0, 1)
    white = (0.7, 0.7, 0.7)

    spheres = [
        sphere(
            center=(-50 + i * 50, 20, 0),
            radius=20,
            material=matte(color),
        )
        for i, color in enumerate((red, green, blue))
    ]

    ground = sphere(
        center=(0, -1000, 0),
        radius=1000,
        material=metallic(white),
    )

    return spheres + [ground]

def config(spx):
    return {
        'samples_per_px': spx,
        'ambiant_color': (0.5, 0.7, 0.9),
        'camera': {
            'look_at': (0, 0, 0),
            'look_from': (150, 100, 200),
        }
    }

render(scene(), **config(100))
