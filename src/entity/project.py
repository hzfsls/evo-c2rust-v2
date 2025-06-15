from __future__ import annotations
import os
import json
import shutil
import time
import subprocess

from entity.metadata import *
from entity.exceptions import RustProjectCompilationFailedError

def create_under_current_dir(dir_path: str, rpath: RustPath):
    if rpath.type == "folder":
        os.makedirs(f"{dir_path}/{rpath.name}", exist_ok=True)
        for k, v in rpath.children.items():
            create_under_current_dir(f"{dir_path}/{rpath.name}", v)
    elif rpath.type == "file":
        with open(f"{dir_path}/{rpath.name}", "w") as f:
            f.write("\n".join(rpath.declarations) + "\n\n")
            for k in rpath.definitions:
                f.write(k.rust_code + "\n\n")
            for k in rpath.macros:
                f.write(k.rust_code + "\n\n")
            for k in rpath.macro_functions:
                f.write(k.rust_code + "\n\n")
            for k in rpath.functions:
                f.write(k.rust_code + "\n\n")

def create_under_current_dir_full(dir_path: str, rpath: RustPath):
    if rpath.type == "folder":
        os.makedirs(f"{dir_path}/{rpath.name}", exist_ok=True)
        for k, v in rpath.children.items():
            create_under_current_dir_full(f"{dir_path}/{rpath.name}", v)
    elif rpath.type == "file":
        with open(f"{dir_path}/{rpath.name}", "w") as f:
            f.write("\n".join(rpath.declarations) + "\n\n")
            for k in rpath.definitions:
                f.write(k.rust_code + "\n\n")
            for k in rpath.macros:
                f.write(k.rust_code + "\n\n")
            for k in rpath.macro_functions:
                f.write(k.rust_code + "\n\n")
            for k in rpath.functions:
                f.write(k.rust_code + "\n\n")


class RustProject:
    def __init__(self, name: str, metadata: RustProjectMetadata, parent_dir="./created_project", template_project_dir="./template_project", no_timestamp=False):
        if no_timestamp:
            self.dir_path = os.path.join(parent_dir, f"{name}")
        else:
            self.dir_path = os.path.join(parent_dir, f"{name}_{int(time.time() * 1000)}")
        self.template_project_dir = template_project_dir
        self.metadata = metadata
        self.create_project()    
    
    def create_project(self):
        os.makedirs(self.dir_path, exist_ok=True)
        shutil.copytree(self.template_project_dir, self.dir_path, dirs_exist_ok=True)
        paths = self.metadata.paths
        for k, v in paths.items():
            create_under_current_dir(os.path.join(self.dir_path, "src"), v)

    def build_project(self):
        result = subprocess.run(["RUSTFLAGS=-Awarnings cargo check"], shell=True, cwd=self.dir_path, timeout=10, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if result.returncode == 0:
            return True, ""
        else:
            error_msg = result.stderr.decode("utf-8")
            return False, error_msg
    
# class RustProjectWithTests(RustProject):
#     def __init__(self, name: str, metadata: RustProjectMetadata, parent_dir="./created_project", template_project_dir="./template_project", no_timestamp=False, rust_tests_dir="./rust_tests"):
#         # todo
#         pass
    
#     def run_project(self):
#         result = subprocess.run(["RUSTFLAGS=-Awarnings cargo test"], shell=True, cwd=self.dir_path, timeout=60, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
#         if result.returncode == 0:
#             return True, ""
#         else:
#             error_msg = result.stderr.decode("utf-8")
#             return False, error_msg


