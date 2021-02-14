from typing import Callable, Dict
import sys
import aoc2020.utils as utils
from aoc2020.prob1.main import main as prob1
from aoc2020.prob2.main import main as prob2
from aoc2020.prob3.main import main as prob3
from aoc2020.prob4.main import main as prob4
from aoc2020.prob5.main import main as prob5
from aoc2020.prob6.main import main as prob6
from aoc2020.prob7.main import main as prob7
from aoc2020.prob8.main import main as prob8
from aoc2020.prob9.main import main as prob9
from aoc2020.prob10.main import main as prob10
from aoc2020.prob11.main import main as prob11
from aoc2020.prob12.main import main as prob12
from aoc2020.prob13.main import main as prob13
from aoc2020.prob14.main import main as prob14

def main() -> int:
  fname = sys.argv[1]

  impl_map: Dict[str, Callable[[utils.Resources], int]] = {
    'prob1': prob1,
    'prob2': prob2,
    'prob3': prob3,
    'prob4': prob4,
    'prob5': prob5,
    'prob6': prob6,
    'prob7': prob7,
    'prob8': prob8,
    'prob9': prob9,
    'prob10': prob10,
    'prob11': prob11,
    'prob12': prob12,
    'prob13': prob13,
    'prob14': prob14
  }

  try:
    resources = utils.create_resource(fname)
  except Exception as e:
    print('Could not load file')
    print(e)
    return 1

  if 'impl' not in resources.config:
    print('Bad config file: implementation not specified')
    return 1

  if resources.config['impl'] not in impl_map:
    print('Invalid implementation specified: {}'.format(resources.config['impl']))
    return 1
  
  impl = impl_map[resources.config['impl']]
  return impl(resources)

if __name__ == '__main__':
  result = main()
  sys.exit(result)
