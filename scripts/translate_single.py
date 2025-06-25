from config.global_config import GlobalConfig
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from llm.client import GenerationClient
from llm.generation import translate_code

import argparse

if __name__ == "__main__":
    code_to_translate = """\
AVLBASE_NODE_S *VosAvlSearchReplaceNodeInLTree(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;
    if (pstNode->pstLeft->pstRight == AVL_NULL_PTR)
    {
        pstReplaceNode = pstNode->pstLeft;
        pstReplaceNode->pstRight = pstNode->pstRight;
        pstReplaceNode->pstRight->pstParent = pstReplaceNode;
        pstReplaceNode->sRHeight = pstNode->sRHeight;
    }
    else
    {
        VosAvlSwapRightMost(pstTree, pstNode->pstLeft, pstNode);
        pstReplaceNode = pstNode->pstLeft;
    }
    return pstReplaceNode;
}
"""
    config = GlobalConfig()
    # 更改你的deepseek api_key信息
    config.api_key = "sk-76da526dbd8b48c3954df9336a8a6592"
    config.base_url = "https://api.deepseek.com/beta"
    config.model_name = "deepseek-coder"
    client = GenerationClient(config)
    result = translate_code(client, "function", code_to_translate)
    print(result)