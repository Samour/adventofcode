from typing import List
import os.path
import yaml

class Resources:

  def __init__(self, config: dict, resource_dir: str):
    self.config = config
    self._resource_dir = resource_dir

  def read_resource(self, r_name: str) -> List[str]:
    with open(os.path.join(self._resource_dir, r_name)) as fh:
      return fh.readlines()


def create_resource(fname: str) -> Resources:
  with open(fname) as fh:
    return Resources(yaml.safe_load(fh), os.path.abspath(os.path.join(fname, '..')))
