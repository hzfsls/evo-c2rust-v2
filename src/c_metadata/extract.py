from __future__ import annotations

import os
import json
import sys

from pathlib import Path

from entity.logger import setup_logger

from c_metadata.preprocess import preprocess
from c_metadata.c_file_metadata import get_metadata

logger = setup_logger("extract.py")

def extract_c_metadata_from_project(proj_name, project_dir, config):
    c_metadata_dir = config.c_metadata_dir
    src_folders = config.src_folders
    macros = config.macros
    replacements = config.replacements
    files = preprocess(
        Path(project_dir, proj_name), src_folders, macros, replacements
    )
    metadata = get_metadata(files)
    declarations_location = {}
    for f in metadata:
        for func in metadata[f].functions:
            declarations_location[func] = f
        for global_var in metadata[f].global_variables:
            declarations_location[global_var] = f
        for type in metadata[f].types:
            if type != "":
                declarations_location[type] = f
    os.makedirs(Path(c_metadata_dir, proj_name), exist_ok=True)
    logger.info(f"C project `{proj_name}` resolve succeeded!")
    try:
        with open(Path(c_metadata_dir, proj_name, "files.json"), "w") as f:
            f.write(
                json.dumps(
                    metadata,
                    default=lambda o: o.__dict__(),
                    indent=4,
                    ensure_ascii=False,
                )
            )
    except FileNotFoundError as e:
        logger.error(f"files.json can not found in {str(Path(c_metadata_dir, proj_name))}.Please check whether the intermediate file has been deleted. \n:{e}")
        sys.exit(1) 
    try:
        with open(
            Path(c_metadata_dir, proj_name, "declarations_location.json"),
            "w",
        ) as f:
            f.write(json.dumps(declarations_location, indent=4, ensure_ascii=False))
    except FileNotFoundError as e:
        logger.error(f"declarations_location.json can not found in {str(Path(c_metadata_dir, proj_name))} \n:{e}")
        sys.exit(1) 

    logger.info(f"C project `{proj_name}` metadata stored at {Path(c_metadata_dir, proj_name)}")