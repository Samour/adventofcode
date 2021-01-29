#!/usr/bin/env python3

import sys
import utils
from prob1.main import main as prob1
from prob2.main import main as prob2

def run() -> int:
  fname = sys.argv[1]

  impl_map = {
    'prob1': prob1,
    'prob2': prob2
  }

  try:
    config = utils.read_yaml(fname)
  except Exception as e:
    print('Could not load file')
    print(e)
    return 1

  if 'impl' not in config:
    print('Bad config file: implementation not specified')
    return 1

  if config['impl'] not in impl_map:
    print('Invalid implementation specified: {}'.format(config['impl']))
    return 1
  
  impl = impl_map[config['impl']]
  return impl(config)

if __name__ == '__main__':
  result = run()
  sys.exit(result)
