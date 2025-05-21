import os
import sys

from rust_metadata.rust_project_creation import (
    RustProject,
    RustCode,
    RustProjectMetadata,
)
from rust_metadata.classes import RustFile

from tqdm import tqdm

import sys

class OptimizationAgent:
    def __init__(self, config, proj_name, metadata, optimize_func, override=False):
        self.name = proj_name
        self.metadata = metadata
        self.optimize_func = optimize_func
        self.override = override
        self.created_project_dir = config.created_project_dir
        self.template_project_dir = config.template_project_dir

    def try_build(self):
        proj = RustProject(self.name, self.metadata, self.created_project_dir, self.template_project_dir)
        success, error_msg = proj.build_project()
        if success:
            return True, ""
        else:
            return False, error_msg

    def try_build_and_get_score(self):
        proj = RustProject(self.name, self.metadata, self.created_project_dir, self.template_project_dir)
        success, error_msg = proj.build_project()
        if success:
            return 0, error_msg
        if "unclosed delimiter" in error_msg:
            return -100000, error_msg
        else:
            return -error_msg.count("\n"), error_msg

    def try_optimize(self, code):
        original_code = code.rust_code
        last_score, _ = self.try_build_and_get_score()
        curr_score = last_score - 1
        curr_code = original_code
        while True:
            candidates = self.optimize_func(curr_code)
            if len(candidates) == 0:
                break
            flag = False
            for c in candidates:
                code.rust_code = c
                new_score, _ = self.try_build_and_get_score()
                if new_score > curr_score or (new_score == curr_score and self.override):
                    curr_score = new_score
                    curr_code = c
                    flag = True
                    break
            if curr_score > last_score or (curr_score == last_score and self.override and flag):
                last_score = curr_score
                curr_code = c
            else:
                break
        code.rust_code = curr_code
        status, error_msg = self.try_build()
        return curr_code, status, error_msg

class OptimizationAgentWithCompilerFeedback(OptimizationAgent):
    def __init__(self, config, proj_name, metadata, optimize_func, max_trial=5, override=False):
        self.max_trial = max_trial
        super().__init__(config, proj_name, metadata, optimize_func, override)

    def try_optimize(self, code):
        original_code = code.rust_code
        last_score, error_msg = self.try_build_and_get_score()
        curr_score = last_score - 1
        curr_code = original_code
        curr_trial = self.max_trial
        while curr_trial > 0:
            curr_trial -= 1
            candidates = self.optimize_func(curr_code, error_msg)
            if len(candidates) == 0:
                break
            flag = False
            for c in candidates:
                code.rust_code = c
                new_score, _ = self.try_build_and_get_score()
                print(f"Try round {self.max_trial - curr_trial} repairing, current score:", new_score)
                if new_score > curr_score or (new_score == curr_score and self.override):
                    curr_score = new_score
                    curr_code = c
                    flag = True
                    break
            if curr_score > last_score or (curr_score == last_score and self.override and flag):
                last_score = curr_score
                curr_code = c
            else:
                break
        code.rust_code = curr_code
        status, error_msg = self.try_build()
        return curr_code, status, error_msg