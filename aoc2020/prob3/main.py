from typing import List
from aoc2020.utils import Resources


TREE_CELL = '#'


class Map:

  def __init__(self):
    self._cells: List[List[str]] = []

  def parse(self, map_lines: List[str]) -> None:
    self._cells = [ list(s.strip()) for s in map_lines ]

  def get_cell(self, x: int, y: int) -> str:
    line = self._cells[y]
    return line[x % len(line)]

  def in_map(self, x: int, y: int) -> bool:
    return y < len(self._cells)


class Positon:

  def __init__(self, slope_x: int, slope_y: int):
    self.x = 0
    self.y = 0
    self._slope_x = slope_x
    self._slope_y = slope_y

  def progress(self) -> None:
    self.x += self._slope_x
    self.y += self._slope_y


def measure_slope(ground_map: Map, position: Positon, debug: bool) -> int:
  trees_c = 0
  while ground_map.in_map(position.x, position.y):
    cell = ground_map.get_cell(position.x, position.y)
    if debug:
      print(cell)
    if cell == TREE_CELL:
      trees_c += 1
    position.progress()

  print('Trees encountered: {}'.format(trees_c))

  return trees_c


def main(resources: Resources) -> int:
  ground_map = Map()
  ground_map.parse(resources.read_resource(resources.config['map_file']))
  debug = 'debug' in resources.config and resources.config['debug']

  mult_result = 1
  for slope in resources.config['slopes']:
    position = Positon(slope['right'], slope['down'])
    mult_result *= measure_slope(ground_map, position, debug)

  print('Multiple of all slopes: {}'.format(mult_result))

  return 0
