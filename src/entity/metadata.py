from __future__ import annotations

import os
import sys
import subprocess
import shutil

import time

from pathlib import Path

from entity.logger import setup_logger

logger = setup_logger("rust_project_creation.py")

class RustProjectMetadata:
    def __init__(self):
        self.paths = {}
    
    def __dict__(self):
        return { k: v.__dict__() for k, v in self.paths.items() }
    
    @staticmethod
    def from_dict(data: dict) -> RustProjectMetadata:
        proj = RustProjectMetadata()
        proj.paths = { k: RustPath.from_dict(v) for k, v in data.items() }
        return proj
    
    def get_all(self, typ: str) -> list[RustFile]:
        all_files = []
        for k, v in self.paths.items():
            all_files.extend(v.recursive_get_all(typ))
        return all_files
    
class RustPath:
    def __init__(self, typ: str):
        self.type = typ # folder or file
    
    @staticmethod
    def from_dict(data: dict) -> RustPath:
        if data["type"] == "folder":
            folder = RustFolder(data["name"])
            folder.children = { k: RustPath.from_dict(v) for k, v in data["children"].items() }
            return folder
        elif data["type"] == "file":
            file = RustFile(data["name"])
            file.declarations = data["declarations"]
            file.definitions = [ RustCode.from_dict(d) for d in data["definitions"] ]
            file.macros = [ RustCode.from_dict(m) for m in data["macros"] ]
            file.macro_functions = [ RustCode.from_dict(mf) for mf in data["macro_functions"] ]
            file.functions = [ RustCode.from_dict(f) for f in data["functions"] ]
            return file
        else:
            raise Exception("Invalid type")

    def recursive_get_all(self, typ: str) -> list[RustFile]:
        if typ == "definition":
            if self.type == "file":
                return self.definitions
            else:
                definitions = []
                for k, v in self.children.items():
                    definitions.extend(v.recursive_get_all(typ))
                return definitions
        elif typ == "macro":
            if self.type == "file":
                return self.macros
            else:
                macros = []
                for k, v in self.children.items():
                    macros.extend(v.recursive_get_all(typ))
                return macros
        elif typ == "macro_function":
            if self.type == "file":
                return self.macro_functions
            else:
                macro_functions = []
                for k, v in self.children.items():
                    macro_functions.extend(v.recursive_get_all(typ))
                return macro_functions
        elif typ == "function":
            if self.type == "file":
                return self.functions
            else:
                functions = []
                for k, v in self.children.items():
                    functions.extend(v.recursive_get_all(typ))
                return functions
        else:
            raise Exception("Invalid type")

class RustFolder(RustPath):
    def __init__(self, name: str):
        super().__init__("folder")
        self.name = name
        self.children = {}
    
    def __dict__(self):
        return {
            "name": self.name,
            "type": self.type,
            "children": { k: v.__dict__() for k, v in self.children.items() }
        }

class RustFile(RustPath):
    def __init__(self, name: str):
        super().__init__("file")
        self.name = name
        self.declarations = []
        self.definitions = []
        self.macros = []
        self.macro_functions = []
        self.functions = []
    
    def __dict__(self):
        return {
            "name": self.name,
            "type": self.type,
            "declarations": self.declarations,
            "definitions": [ d.__dict__() for d in self.definitions ],
            "macros": [ m.__dict__() for m in self.macros ],
            "macro_functions": [ mf.__dict__() for mf in self.macro_functions ],
            "functions": [ f.__dict__() for f in self.functions ]
        }

class RustCode:
    def __init__(self, c_code: str):
        self.c_code = c_code
        self.rust_code = ""
    
    def __dict__(self):
        return {
            "c_code": self.c_code,
            "rust_code": self.rust_code
        }

    @staticmethod
    def from_dict(data: dict) -> RustCode:
        code = RustCode(data["c_code"])
        code.rust_code = data["rust_code"]
        return code

def create_under_current_dir(dir_path: str, rpath: RustPath):
    if rpath.type == "folder":
        os.makedirs(f"{dir_path}/{rpath.name}", exist_ok=True)
        for k, v in rpath.children.items():
            create_under_current_dir(f"{dir_path}/{rpath.name}", v)
    elif rpath.type == "file":
        try:
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
        except FileNotFoundError as e:
            logger.error(f"{dir_path}/{rpath.name} can not found.Please check whether the intermediate file has been deleted.\n:{e}")
            sys.exit(1) 

class RustProject:
    def __init__(self, name: str, metadata: RustProjectMetadata, parent_dir="./created_project", template_project_dir="./template_project", no_timestamp=False):
        if no_timestamp:
            self.dir_path = Path(parent_dir, f"{name}")
        else:
            self.dir_path = Path(parent_dir, f"{name}_{int(time.time() * 1000)}")
        self.template_project_dir = template_project_dir
        self.metadata = metadata
        self.create_project()    
    
    def create_project(self):
        os.makedirs(self.dir_path, exist_ok=True)
        shutil.copytree(self.template_project_dir, self.dir_path, dirs_exist_ok=True)
        paths = self.metadata.paths
        for k, v in paths.items():
            create_under_current_dir(Path(self.dir_path, "src"), v)


    def build_project(self):
        if sys.platform.startswith('win'):
            result = subprocess.run('set RUSTFLAGS=-Awarnings && cargo check', shell=True, cwd=self.dir_path, timeout=10, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        elif sys.platform.startswith('linux'):
            result = subprocess.run(["RUSTFLAGS=-Awarnings cargo check"], shell=True, cwd=self.dir_path, timeout=10, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if result.returncode == 0:
            return True, ""
        else:
            try:
                error_msg = result.stderr.decode("utf-8")
            except UnicodeDecodeError as utf8_error:
                try:
                    error_msg = result.stderr.decode("gbk")
                except UnicodeDecodeError as gbk_error:
                    raise Exception(
                        f"Failed to decode result.stderr with both UTF-8 and GBK. "
                        f"UTF-8 error: {utf8_error}, GBK error: {gbk_error}"
                    ) from gbk_error
            except Exception as e:
                raise Exception(f"Unexpected error decoding result.stderr: {e}") from e
            return False, error_msg