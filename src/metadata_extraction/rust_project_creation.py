from __future__ import annotations
import os
import json
import shutil
import time
import subprocess

from entity.metadata import *
from metadata_extraction.rust_metadata import resolve_metadata
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
    
class RustProjectWithTests(RustProject):
    def __init__(self, name: str, metadata: RustProjectMetadata, parent_dir="./created_project", template_project_dir="./template_project", no_timestamp=False, rust_tests_dir="./rust_tests"):
        # todo
        pass
    
    def run_project(self):
        result = subprocess.run(["RUSTFLAGS=-Awarnings cargo test"], shell=True, cwd=self.dir_path, timeout=60, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if result.returncode == 0:
            return True, ""
        else:
            error_msg = result.stderr.decode("utf-8")
            return False, error_msg


def c_metadata_to_rust_metadata(global_config):
    proj_name = global_config.project_name
    c_metadata_dir = global_config.c_metadata_dir
    rust_metadata_dir = global_config.rust_metadata_dir
    created_project_dir = global_config.created_project_dir
    template_project_dir = global_config.template_project_dir    
    with open(os.path.join(c_metadata_dir, proj_name, "files.json"), "r") as f:
        files_data = json.load(f)
    with open(
        os.path.join(c_metadata_dir, proj_name, "declarations_location.json"),
        "r",
    ) as f:
        declarations_data = json.load(f)
    metadata = resolve_metadata(files_data, declarations_data)
    os.makedirs(os.path.join(rust_metadata_dir, proj_name), exist_ok=True)
    with open(os.path.join(rust_metadata_dir, proj_name, "metadata.json"), "w") as f:
        json.dump(metadata.__dict__(), f, indent=4)

    with open(os.path.join(rust_metadata_dir, proj_name, "metadata.json"), "r") as f:
        files_data = json.load(f)
    metadata = RustProjectMetadata.from_dict(files_data)
    print(
        f"Rust project `{proj_name}` metadata stored at {os.path.join(c_metadata_dir, proj_name)}"
    )
    proj = RustProject(proj_name, metadata, created_project_dir, template_project_dir)
    print(f"Create rust project `{proj_name}` at {proj.dir_path}")
    success, error_msg = proj.build_project()
    if success:
        print(
            f"Rust skeleton project {proj_name}(at {proj.dir_path}) build succeeded!")
    else:
        raise RustProjectCompilationFailedError(error_msg)
    return metadata