import _trt

def matte(color):
    return _trt.Material.matte(color)

def metallic(color, fuzz=0):
    return _trt.Material.metallic_fuzzed(color, float(fuzz))

def dielectric(ref_idx):
    return _trt.Material.dielectric(ref_idx)

def diffuse_color(color):
    return _trt.Material.diffuse_color(color)

def image(url, cors_proxy=False):
    if cors_proxy:
        url = f'https://cors-anywhere.herokuapp.com/{url}'
    return _trt.Material.image(url)
