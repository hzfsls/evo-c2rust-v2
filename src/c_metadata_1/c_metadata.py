from __future__ import annotations
import tree_sitter_c as ts_c
from typing import Generator
from tree_sitter import Language, Parser, Tree, Node
import os
import json

from pathlib import Path

import sys
from c_metadata.preprocess import preprocess
from entity.logger import setup_logger

c_language = Language(ts_c.language())
c_parser = Parser(c_language)

logger = setup_logger("c_metadata.py")

def get_metadata(files: dict[str, str]) -> dict[str, CFileMetadata]:
    result = {}
    for f in files:
        result[f] = CFileMetadata.from_code(f, files[f])
    return result

def has_identifier(node: Node):
    chind_cnt = node.child_count
    if node.type == "identifier" or node.type == "type_identifier":
        return True, node.text.decode("utf-8")
    if chind_cnt == 0:
        return False, None
    for i in range(chind_cnt):
        res, name = has_identifier(node.child(i))
        if res:
            return True, name
    return False, None


def has_function_declarator(node: Node):
    chind_cnt = node.child_count
    if node.type == "function_declarator":
        res, name = has_identifier(node)
        assert res, node.text.decode("utf-8").strip()
        return res, name
    if chind_cnt == 0:
        return False, None
    for i in range(chind_cnt):
        res, name = has_function_declarator(node.child(i))
        if res:
            return True, name
    return False, None

def has_init_declarator(node: Node):
    chind_cnt = node.child_count
    if node.type == "init_declarator":
        res, name = has_identifier(node)
        assert res, node.text.decode("utf-8").strip()
        return res, name
    if chind_cnt == 0:
        return False, None
    for i in range(chind_cnt):
        res, name = has_init_declarator(node.child(i))
        if res:
            return True, name
    return False, None


def has_extern(node: Node):
    chind_cnt = node.child_count
    if node.type == "extern":
        return True
    if chind_cnt == 0:
        return False
    for i in range(chind_cnt):
        res= has_extern(node.child(i))
        if res:
            return True
    return False


def get_name_and_class_from_types_specifier(node: Node):
    chind_cnt = node.child_count
    assert chind_cnt >= 2, node.text.decode("utf-8").strip()
    if node.type == "struct_specifier":
        assert node.child(0).type == "struct", node.text.decode("utf-8").strip()
        assert node.child(1).type == "type_identifier" or node.child(1).type == "field_declaration_list", node.text.decode("utf-8").strip()
        if node.child(1).type == "type_identifier":            
            if chind_cnt >= 3:
                assert node.child(2).type == "field_declaration_list", node.text.decode("utf-8").strip()
                return "type", node.child(1).text.decode("utf-8").strip()
            else:
                return "declaration", node.child(1).text.decode("utf-8").strip()
        else:
            return "type", ""
    elif node.type == "union_specifier":
        assert node.child(0).type == "union", node.text.decode("utf-8").strip()
        assert node.child(1).type == "type_identifier" or node.child(1).type == "field_declaration_list", node.text.decode("utf-8").strip() 
        if node.child(1).type == "type_identifier":            
            if chind_cnt >= 3:
                assert node.child(2).type == "field_declaration_list", node.text.decode("utf-8").strip()
                return "type", node.child(1).text.decode("utf-8").strip()
            else:
                return "declaration", node.child(1).text.decode("utf-8").strip()
        else:
            return "type", ""
    elif node.type == "enum_specifier":
        assert node.child(0).type == "enum", node.text.decode("utf-8").strip()
        assert node.child(1).type == "type_identifier" or node.child(1).type == "enumerator_list", node.text.decode("utf-8").strip()
        if node.child(1).type == "type_identifier":            
            if chind_cnt >= 3:
                assert node.child(2).type == "enumerator_list", node.text.decode("utf-8").strip()
                return "type", node.child(1).text.decode("utf-8").strip()
            else:
                return "declaration", node.child(1).text.decode("utf-8").strip()
        else:
            return "type", ""
    else:
        raise ValueError(node.text.decode("utf-8").strip())

def expand_node_tree_to_seq(node: Node):
    seq = []
    child_cnt = node.child_count
    if child_cnt == 0:
        return [node]
    else:
        for i in range(child_cnt):
            seq.extend(expand_node_tree_to_seq(node.child(i)))
        return seq

def resolve_typedef(node: Node):
    chind_cnt = node.child_count
    assert node.child(0).type == "typedef" and node.child(1).type in ["type_identifier", "primitive_type", "sized_type_specifier", "struct_specifier", "union_specifier", "enum_specifier", "primitive_type"], node.text.decode("utf-8").strip()
    flag, func_name = has_function_declarator(node)
    if flag:
        res, name = has_identifier(node)
        assert res, node.text.decode("utf-8").strip()
        return "", name
    else:
        seq = expand_node_tree_to_seq(node)
        assert seq[-1].type == ";", node.text.decode("utf-8").strip()
        assert seq[-2].type == "type_identifier", node.text.decode("utf-8").strip()
        decl_name = None
        if node.child(1).type == "type_identifier":
            decl_name = node.child(1).text.decode("utf-8").strip()
        elif node.child(1).type in ["struct_specifier", "union_specifier", "enum_specifier"]:
            _, decl_name = get_name_and_class_from_types_specifier(node.child(1))
        real_name = seq[-2].text.decode("utf-8").strip()
        return decl_name, real_name



class CFileMetadata:
    def __init__(self, name: str):
        self.name = name
        self.includes = []
        self.types = {"":[], }
        self.macros = []
        self.macro_functions = []
        self.global_variables = {}
        self.declarations = set()
        self.functions = {}

    def __str__(self):
        return str(self.__dict__())
    
    def __dict__(self):
        return {
            "includes": [x for x in self.includes],
            "macros": [x for x in self.macros],
            "macro_functions": [x for x in self.macro_functions],
            "types":  self.types,
            "global_variables": self.global_variables,
            "declarations": list(self.declarations),
            "functions": self.functions
        }
    
    @staticmethod
    def from_code(name:str, code: str) -> CFileMetadata:
        tree = c_parser.parse(bytes(code, "utf8"))
        data = CFileMetadata(name)
        data.parse_node(tree.root_node)
        return data 
    
    def parse_node(self, node: Node):
        chind_cnt = node.child_count
        snippet = node.text.decode("utf-8").strip()
        if node.type == "preproc_def":
            self.macros.append(snippet)
        elif node.type == "preproc_function_def":
            self.macro_functions.append(snippet)
        elif node.type == "preproc_include":
            self.includes.append(snippet)
        elif node.type == "type_definition":
            decl_name, name = resolve_typedef(node)
            if name == "":
                self.types[name].append(snippet)
            else:
                self.types[name] = snippet
            if decl_name is not None and decl_name != "":
                self.declarations.add(decl_name)
        elif node.type in ["enum_specifier", "struct_specifier", "union_specifier"]:
            clas, name = get_name_and_class_from_types_specifier(node)
            if clas == "type":
                if not snippet.endswith(";"):
                    snippet += ";"
                if name == "":
                    self.types[name].append(snippet)
                else:
                    self.types[name] = snippet
            else:
                self.declarations.add(name)
        elif node.type == "function_definition":
            res, name = has_function_declarator(node)
            assert res, "B"
            self.functions[name] = snippet
        elif node.type == "declaration":
            res, name = has_function_declarator(node)
            if not res:
                flag = has_extern(node)
                if flag:
                    flag, name = has_identifier(node)
                    assert flag == True, snippet
                    self.declarations.add(name)
                else:
                    flag, name = has_init_declarator(node)
                    assert flag == True, snippet
                    if flag:
                        self.global_variables[name] = snippet
                    else:
                        logger.warning(f"`{snippet}` is neither an extern declaration or global variable initialization.")
            else:
                self.declarations.add(name)

        else:
            chind_cnt = node.child_count
            if chind_cnt == 0:
                return
            else:
                for i in range(chind_cnt):
                    self.parse_node(node.child(i))
    

def extract_c_metadata_from_project(proj_name, project_dir, c_metadata_dir, src_folders, macros = {}, replacements = {}):
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