import _trt

def matte(color):
    return _trt.Material.matte(color)

def metallic(color):
    return _trt.Material.metallic(color)

def dielectric(ref_idx):
    return _trt.Material.dielectric(ref_idx)

def diffuse_color(color):
    return _trt.Material.diffuse_color(color)

def image(url):
    return _trt.Material.image(url)
