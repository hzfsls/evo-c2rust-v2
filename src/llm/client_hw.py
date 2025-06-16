from openai import OpenAI
from httpx import Client

from entity.exceptions import CallLLMTimeoutError

import os
proxies = ['http_proxy', 'https_proxy', 'HTTP_PROXY', 'HTTPS_PROXY']
for p in proxies:
    if p in os.environ:
        del os.environ[p]
os.environ['NO_PROXY'] = 'http://api.openai.rnd.huawei.com/v1'

class GenerationClient:
    def __init__(self, config):
        self.config = config
        self.api_key = config.api_key
        self.base_url =  config.base_url
        self.model_name = config.model_name
    
    def get_response(self, text):
        openai_client = OpenAI(
            api_key=self.api_key,
            base_url=self.base_url
        )
        max_trail = 10
        while max_trail > 0:
            try:
                response = openai_client.chat.completions.create(
                    model=self.model_name,
                    messages=[
                        {"role": "user", "content": text + '/no_think'},
                    ],
                    stop=["```\n"],
                    temperature=0,
                    top_p=0.01,
                    max_tokens=4096,
                    stream=False,
                )
                break
            except Exception as e:
                max_trail -= 1
                if max_trail == 0:
                    raise CallLLMTimeoutError(f"Failed to call LLM after 10 attempts: {e}")
        result = response.choices[0].message.content.strip()
        if result.startswith("```rust"):
            result = result[7:].strip()
        if result.endswith("```"):
            result = result[:-3].strip()
        return result
