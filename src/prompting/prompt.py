# write a prompting class that get the prompt message after giving the codes
    
class FixedDefinitionTranslatePrompt:
    def __init__(self):
        super().__init__()
    
    def get_prompt(self, code: str) -> str:
        prompt = ""
        return prompt

class FixedMacroTranslatePrompt:
    def __init__(self):
        super().__init__()
    
    def get_prompt(self, code: str) -> str:
        prompt = ""
        return prompt

class FixedMacroFunctionTranslatePrompt:
    def __init__(self):
        super().__init__()
    
    def get_prompt(self, code: str) -> str:
        prompt = ""
        return prompt

class FixedDummyFunctionTranslatePrompt:
    def __init__(self):
        super().__init__()
    
    def get_prompt(self, code: str) -> str:
        prompt = ""
        return prompt

class FixedFunctionTranslatePrompt:
    def __init__(self):
        super().__init__()
    
    def get_prompt(self, code: str) -> str:
        prompt = ""
        return prompt

class FixedFunctionRepairPrompt:
    def __init__(self):
        super().__init__()
    
    def get_prompt(self, code: str, error_msg: str) -> str:
        prompt = f"Fix the following C code:\n\n{code}\n\nError message: {error_msg}\n\n"
        return prompt

class FixedRefactorPrompt:
    def __init__(self):
        super().__init__()
    
    def get_prompt(self, code: str, task: str) -> str:
        prompt = f"Refactor the following C code:\n\n{code}\n\nTask: {task}\n\n"
        return prompt