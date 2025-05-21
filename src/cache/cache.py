from __future__ import annotations

import sys
import os
import json
from pathlib import Path

from entity.logger import setup_logger

from entity.metadata import RustPath

logger = setup_logger("cache.py")

class Cache:
    def __init__(self, path: str):
        self.path = path
        self.cache = {}
        self.cache_index = {}

        if not os.path.exists(self.path):
            os.makedirs(self.path)
            with open(Path(self.path, f"index.json"), "w") as f:
                json.dump({}, f)
        else:
            with open(Path(self.path, f"index.json"), "r") as f:
                self.cache_index = json.load(f)
            for k, path in self.cache_index.items():
                # find files in the folder from 0.rs, 1.rs, ..., until not found
                curr_idx = 0
                self.cache[k] = []
                while True:
                    path = Path(self.path, str(path), f"{curr_idx}.rs")
                    if not os.path.exists(path):
                        break
                    curr_idx += 1
                    with open(path, "r") as f:
                        self.cache[k].append(f.read())
    
    def get(self, key):
        if key in self.cache_index:
            return self.cache[key]
        else:
            logger.error(f"Key {key} not found in cache.")
            sys.exit(1)
    
    def update(self, key, values):
        if key in self.cache_index:
            new_idx = self.cache_index[key]
            # delete the old files
            if os.path.exists(Path(self.path, str(new_idx))):
                for file in os.listdir(Path(self.path, str(new_idx))):
                    os.remove(Path(self.path, str(new_idx), file))
        else:
            new_idx = str(len(self.cache))
            os.makedirs(Path(self.path, str(new_idx)))
        self.cache_index[key] = new_idx
        self.cache[key] = values

        for idx, value in enumerate(values):
            with open(Path(self.path, str(new_idx), f"{idx}.rs"), "w") as f:
                f.write(value)
        
        # dump index.json
        with open(Path(self.path, f"index.json"), "w") as f:
            json.dump(self.cache_index, f)