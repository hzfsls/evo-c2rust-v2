pdm run scripts/code_generation.py --project_name avl
pdm run scripts/code_verification.py --project_name avl
pdm run scripts/code_optimize_fix_delim.py --project_name avl
pdm run scripts/code_optimize_rule_based.py --project_name avl
pdm run scripts/code_optimize_llm_repair.py --project_name avl