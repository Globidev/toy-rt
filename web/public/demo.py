from trt import Sphere, Rect, Scene, Camera, Material

red = (0.9, 0.3, 0.3)
green = (0.3, 0.9, 0.3)
blue = (0.3, 0.3, 0.9)
white = (0.7, 0.7, 0.7)

def scene():
    spheres = [
        Sphere(
            center=(150+ i * 150, 150 + i * 100, 150 + i * 100),
            radius=80,
            material=Material.metallic(color),
        )
        for i, color in enumerate((red, green, blue))
    ]

    cornell_box = [
        # side walls
        Rect(y=(0, 600), z=(-1000, 600), x=600, material=Material.metallic(red)).flip_normals(),
        Rect(y=(0, 600), z=(-1000, 600), x=0, material=Material.matte(blue)),

        # ceiling + light
        Rect(x=(0, 600), z=(-1000, 600), y=600, material=Material.dielectric(0.5)).flip_normals(),
        Rect(x=(100, 500), z=(-100, 350), y=599, material=Material.diffuse_color((5, 5, 5))),
        # floor
        Rect(x=(0, 600), z=(-1000, 600), y=0, material=Material.matte(green)),

        # front
        Rect(x=(0, 600), y=(0, 600), z=600, material=Material.metallic(white)).flip_normals(),
        # back
        Rect(x=(0, 600), y=(0, 600), z=-1000, material=Material.metallic(white)),
    ]

    return Scene(**{
        "world": cornell_box + spheres,
        "width": 400,
        "height": 400,
        "samples_per_px": 150,
        "rays_per_sample": 10,
        "camera": Camera(
            look_at=(300, 300, 0),
            look_from=(300, 300, -800),
        )
    })
