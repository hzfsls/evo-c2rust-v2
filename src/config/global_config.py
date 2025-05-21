# Define a global configuration class for the project
# every other module can access the configuration values by a singleton instance

import configparser
import os

from config.logger import setup_logger

def read_config():
    config = configparser.ConfigParser()
    config.read(os.path.join(os.path.dirname(__file__), 'config.ini'))
    return config

class GlobalConfig:
    def __init__(self):
        config = read_config()
        self.project_name = None
        self.
        

    def set(self, key, value):
        if key in self.config['Paths']:
            self.config['Paths'][key] = value
            with open(os.path.join(os.path.dirname(__file__), 'config.ini'), 'w') as configfile:
                self.config.write(configfile)
        else:
            raise KeyError(f"Key {key} not found in configuration.")
    
