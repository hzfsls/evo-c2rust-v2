import subprocess
import os
import json
import sys

from pathlib import Path
import tree_sitter_c as ts_c
from typing import Generator
from tree_sitter import Language, Parser, Node
import sys

from entity.logger import setup_logger

c_language = Language(ts_c.language())
c_parser = Parser(c_language)

logger = setup_logger("preprocess.py")

def get_files(src_folder: str, dirs: list[str]) -> dict[str, str]:
    files = {}
    for d in dirs:
        for root, _, fs in os.walk(Path(src_folder, d)):
            for f in fs:
                if f.endswith(".c") or f.endswith(".h"):
                    file_name = Path(root, f)
                    try:
                        file_content = open(file_name, errors='ignore').read()  
                    except FileNotFoundError as e:
                        logger.error(f"{file_name} can not found.Please check whether the intermediate file has been deleted. \n:{e}")
                        sys.exit(1)
                    new_file_name = os.path.relpath(file_name, src_folder)
                    files[new_file_name] = file_content
    return files

def remove_files_comments(files: dict[str, str]) -> dict[str, str]:
    results = {}
    for file_rel_path in files:
        tmp_file_path = Path(".tmp", file_rel_path)
        os.makedirs(os.path.dirname(tmp_file_path), exist_ok=True)
        try:
            with open(tmp_file_path, "w") as f:
                f.write(files[file_rel_path].replace("\\\n", ""))
        except FileNotFoundError as e:
            logger.error(f" {tmp_file_path} can not found.Please check whether the intermediate file has been deleted. \n:{e}")
            sys.exit(1) 
        result = subprocess.run(f"gcc -fpreprocessed -dD -E {tmp_file_path}", shell=True, capture_output=True)
        if result.returncode != 0:
            raise Exception(f"Error in removing {file_rel_path} comments")
        try:
            results[file_rel_path] = result.stdout.decode("utf-8")
        except Exception as e:
            raise Exception(f"Error in decoding {file_rel_path} output: {e}")
    return results

def clang_format_files(files: dict[str, str]) -> dict[str, str]:
    results = {}
    for file_rel_path in files:
        tmp_file_path = Path(".tmp", file_rel_path)
        os.makedirs(os.path.dirname(tmp_file_path), exist_ok=True)
        try:
            with open(tmp_file_path, "w") as f:
                f.write(files[file_rel_path])
        except FileNotFoundError as e:
            logger.error(f"{tmp_file_path} can not found.Please check whether the intermediate file has been deleted. \n:{e}")
            sys.exit(1) 
        try:
            result = subprocess.run(f"clang-format -style=Microsoft {tmp_file_path}", 
                                shell=True, 
                                capture_output=True, 
                                check=True)
        except Exception as e:
            logger.error(f"Please check if clang-format is installed. \n:{e}")
            sys.exit(1)

        if result.returncode != 0:
            raise Exception(f"Error in removing {file_rel_path} comments")
        try:
            results[file_rel_path] = result.stdout.decode("utf-8")
        except UnicodeDecodeError as utf8_error:
            try:
                results[file_rel_path] = result.stdout.decode("gbk")
            except UnicodeDecodeError as gbk_error:
                raise Exception(
                    f"Failed to decode {file_rel_path} with both UTF-8 and GBK. "
                    f"UTF-8 error: {utf8_error}, GBK error: {gbk_error}"
                ) from gbk_error
        except Exception as e:
            raise Exception(f"Unexpected error decoding {file_rel_path}: {e}") from e
    return results

def replace_files_macros(files: dict[str, str], macros: dict[str, str] = {}, replacements: dict[str, str] = {}) -> dict[str, str]:
    results = {}
    macros = dict(sorted(macros.items(), key=lambda x: len(x[0]), reverse=True))
    for file_rel_path in files:
        content = files[file_rel_path]
        for k, v in replacements.items():
            content = content.replace(k, v)
        for macro, replace in macros.items():
            lines = content.split("\n")
            new_lines = []
            for line in lines:
                if not line.strip().startswith("#"):
                    new_lines.append(line.replace(macro, replace))
                else:
                    new_lines.append(line)
            content = "\n".join(new_lines)
        results[file_rel_path] = content
    return results

def if_parse_error(node: Node) -> bool:
    if node.type in ["gnu_asm_expression", "function_definition"]:
        return True
    if node.type == "ERROR":
        return False
    for i in range(node.child_count):
        child = node.child(i)
        if not if_parse_error(child):
            return False
    return True

def try_parse(files: dict[str, str]):
    for file_name, file_content in files.items():
        tree = c_parser.parse(bytes(file_content, "utf-8"))
        if not if_parse_error(tree.root_node):
            logger.error(f"Error in parsing {file_name}. \nfile_content:{file_content}")
            sys.exit(1)

def preprocess(src_folder: str, dirs: list[str], macros: dict[str, str] = {}, replacements: dict[str, str] = {}) -> dict[str, str]:
    files = get_files(src_folder, dirs)
    files = remove_files_comments(files)
    files = replace_files_macros(files, macros, replacements)
    files = clang_format_files(files)
    try_parse(files)
    return files