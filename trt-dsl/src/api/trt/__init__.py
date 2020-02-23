import _trt

from . import shape
from . import material

def render(world, **config):
    DEFAULT_WIDTH = 500
    DEFAULT_HEIGHT = 500
    DEFAULT_SPX = 50
    DEFAULT_RPS = 25
    DEFAULT_AMBIANT = (0, 0, 0)
    DEFAULT_CAMERA = {
        'look_at': (0, 0, 0),
        'look_from': (0, 0, 0)
    }

    config = {
        'world': world,
        'width': config.get('width', DEFAULT_WIDTH),
        'height': config.get('height', DEFAULT_HEIGHT),
        'samples_per_px': config.get('samples_per_px', DEFAULT_SPX),
        'rays_per_sample': config.get('rays_per_sample', DEFAULT_RPS),
        'ambiant_color': config.get('ambiant_color', DEFAULT_AMBIANT),
        'camera': _camera(**config.get('camera', DEFAULT_CAMERA))
    }

    _trt.__render_scene = _trt.Scene(**config)

def _camera(look_from, look_at):
    return _trt.Camera(look_from, look_at)

def rand():
    return _trt.rand()
