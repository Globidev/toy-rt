import trt

red = (1, 0, 0)
green = (0, 1, 0)
blue = (0, 0, 1)

def scene():
    spheres = [
        trt.Sphere(
            center=(-50 + i * 50, 0, 0),
            radius=20,
            color=color,
        )
        for i, color in enumerate((red, green, blue))
    ]

    ground = trt.Sphere(
        center=(0, -5050, 0),
        radius=5000,
        color=(1, 1, 1),
    )

    return trt.Scene(**{
        "world": spheres + [ground]
    })
