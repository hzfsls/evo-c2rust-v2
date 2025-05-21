from __future__ import annotations
import os
import json
import shutil
import time
import subprocess

from pathlib import Path

import sys
from entity.metadata import *
from rust_metadata.rust_metadata import resolve_metadata
from entity.exceptions import RustProjectCompilationFailedError
from entity.logger import setup_logger

logger = setup_logger("rust_project_creation.py")


def create_under_current_dir_full(dir_path: str, rpath: RustPath):
    if rpath.type == "folder":
        os.makedirs(f"{dir_path}/{rpath.name}", exist_ok=True)
        for k, v in rpath.children.items():
            create_under_current_dir_full(f"{dir_path}/{rpath.name}", v)
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


def c_metadata_to_rust_metadata(proj_name, c_metadata_dir="./c_metadata", rust_metadata_dir="./rust_metadata", created_project_dir="./created_project", template_project_dir="./template_project"):
    try:
        with open(Path(c_metadata_dir, proj_name, "files.json"), "r") as f:
            files_data = json.load(f)
    except FileNotFoundError as e:
        logger.error(f"files.json can not found in {str(Path(c_metadata_dir, proj_name))}.Please check whether the intermediate file has been deleted.\n:{e}")
        sys.exit(1)
    except json.JSONDecodeError as e:
        logger.error(f"files.json in {str(Path(c_metadata_dir, proj_name))} decoding failed.Please check whether the intermediate file has been changed.\n:{e}")
        sys.exit(1)
    try:
        with open(
            Path(c_metadata_dir, proj_name, "declarations_location.json"),
            "r",
        ) as f:
            declarations_data = json.load(f)
    except FileNotFoundError as e:
        logger.error(f"declarations_location.json can not found in {str(Path(c_metadata_dir, proj_name))}.Please check whether the intermediate file has been deleted.\n:{e}")
        sys.exit(1)
    except json.JSONDecodeError as e:
        logger.error(f"declarations_location.json in {str(Path(c_metadata_dir, proj_name))} decoding failed.Please check whether the intermediate file has been changed.\n:{e}")
        sys.exit(1)
    metadata = resolve_metadata(files_data, declarations_data)
    os.makedirs(Path(rust_metadata_dir, proj_name), exist_ok=True)
    try:
        with open(Path(rust_metadata_dir, proj_name, "metadata.json"), "w") as f:
            json.dump(metadata.__dict__(), f, indent=4)
    except FileNotFoundError as e:
        logger.error(f"metadata.json can not found in {str(Path(rust_metadata_dir, proj_name))}.Please check whether the intermediate file has been deleted.\n:{e}")
        sys.exit(1)
    try:
        with open(Path(rust_metadata_dir, proj_name, "metadata.json"), "r") as f:
            files_data = json.load(f)
    except FileNotFoundError as e:
        logger.error(f"metadata.json can not found in {str(Path(rust_metadata_dir, proj_name))}.Please check whether the intermediate file has been deleted.\n:{e}")
        sys.exit(1)
    except json.JSONDecodeError as e:
        logger.error(f"metadata.json in {str(Path(rust_metadata_dir, proj_name))} decoding failed.Please check whether the intermediate file has been changed.\n:{e}")
        sys.exit(1)
    metadata = RustProjectMetadata.from_dict(files_data)
    logger.info(f"Rust project `{proj_name}` metadata stored at {Path(c_metadata_dir, proj_name)}")
    proj = RustProject(proj_name, metadata, created_project_dir, template_project_dir)
    logger.info(f"Create rust project `{proj_name}` at {proj.dir_path}")
    success, error_msg = proj.build_project()
    if success:
        logger.info(f"Rust skeleton project {proj_name}(at {proj.dir_path}) build succeeded!")
    else:
        raise RustProjectCompilationFailedError(error_msg)
    return metadata