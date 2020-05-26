import _trt
import types

def _isfloatlike(x):
    return isinstance(x, int) or isinstance(x, float)

def _map_range(r):
    return tuple(float(x) for x in r)

def sphere(center, radius, material):
    return _trt.Shape.sphere(_map_range(center), float(radius), material)

def cylinder(base, height, radius, material):
    return _trt.Shape.cylinder(_map_range(base), float(height), float(radius), material)

def rect(x, y, z, material):
    def validate_rect_args(r1, r2, f):
        correct_outer_types = isinstance(r1, tuple) and\
                              isinstance(r2, tuple) and\
                              _isfloatlike(f)

        if not correct_outer_types:
            return False

        correct_inner_types = len(r1) == 2 and len(r2) == 2 and\
                              _isfloatlike(r1[0]) and _isfloatlike(r1[1]) and\
                              _isfloatlike(r2[0]) and _isfloatlike(r2[1])

        if not correct_inner_types:
            raise WrongRectArgumentError

        return True

    if validate_rect_args(x, y, z):
        return _trt.Shape.xy_rect(*_map_range(x), *_map_range(y), float(z), material)
    if validate_rect_args(x, z, y):
        return _trt.Shape.xz_rect(*_map_range(x), *_map_range(z), float(y), material)
    if validate_rect_args(y, z, x):
        return _trt.Shape.yz_rect(*_map_range(y), *_map_range(z), float(x), material)

    raise WrongRectArgumentError

class WrongRectArgumentError(Exception):
    pass

def bvh_node(hits):
    if isinstance(hits, types.GeneratorType):
        hit_list = list(hits)
    else:
        hit_list = hits

    return _trt.Shape.bvh_node(hit_list)

def hitbox(min, max, material):
    return _trt.Shape.hitbox(min, max, material)
