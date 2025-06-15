import os
import json

class LLMGenerationCache:
    def __init__(self, cache_dir, name):
        self.path = os.path.join(cache_dir, name)
        self.cache_index = {}
        self.cache = {}
        if not os.path.exists(self.path):
            os.makedirs(self.path)
            with open(os.path.join(self.path, f"index.json"), "w") as f:
                json.dump({}, f)
        else:
            with open(os.path.join(self.path, f"index.json"), "r") as f:
                self.cache_index = json.load(f)
            for k, path in self.cache_index.items():
                with open(os.path.join(self.path, path, "result.rs"), "r") as f:
                    self.cache[k] = f.read()

    def update(self, key, value):
        if key in self.cache_index:
            new_idx = self.cache_index[key]
        else:
            new_idx = str(len(self.cache))
            os.makedirs(os.path.join(self.path, str(new_idx)))
        self.cache_index[key] = new_idx
        self.cache[key] = value
        with open(os.path.join(self.path, str(new_idx), "result.rs"), "w") as f:
            f.write(value)
        with open(os.path.join(self.path, f"index.json"), "w") as f:
            json.dump(self.cache_index, f)