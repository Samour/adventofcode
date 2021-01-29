import yaml

def read_yaml(fname):
  with open(fname) as fh:
    return yaml.safe_load(fh)
