import os
import json

SCENES_DIR = "../trt-dsl/scenes"
OUT_FILE = "./src/assets/scenes.ts"
EXCEPTIONS = (
    'dynamic'
)

scenes = {
    filename[:-3].lower().replace('_', ' '): open(f'{dirpath}/{filename}').read()
    for (dirpath, _, files) in os.walk(SCENES_DIR)
    for filename in files
    if filename.endswith('.py')
}

filtered_scenes = {
    k: v
    for k, v in scenes.items()
    if  k not in EXCEPTIONS
}

scenes_data = json.dumps(filtered_scenes, sort_keys=True, indent=2)
ts_code = f'export const scenes: {{ [name: string]: string }} = {scenes_data}'

open(OUT_FILE, 'w').write(ts_code)
