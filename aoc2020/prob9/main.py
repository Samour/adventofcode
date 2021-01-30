from typing import List
from aoc2020.utils import Resources


class XMASScanner:

  def __init__(self, scan_length: int):
    self._scan_length = scan_length
    self._preceeding_values: List[int] = []

  def _validate_value(self, value: int) -> bool:
    if len(self._preceeding_values) < self._scan_length:
      return True
    for s in self._preceeding_values:
      if s <= value and value - s != s and value - s in self._preceeding_values:
        return True

    return False

  def receive_value(self, value: int) -> bool:
    valid = self._validate_value(value)
    self._preceeding_values.append(value)
    if len(self._preceeding_values) > self._scan_length:
      self._preceeding_values.pop(0)

    return valid


def find_first_incorrect_value(values: List[int], scan_length: int) -> int:
  scanner = XMASScanner(scan_length)
  for v in values:
    if not scanner.receive_value(v):
      return v

  raise Exception('Could not find any invalid value')


def find_contiguous_sum_to_value(values: List[int], target: int) -> List[int]:
  for i in range(len(values)):
    for j in range(i + 1, len(values)):
      if sum(values[i:j]) == target:
        return values[i:j]

  raise Exception('Could not find values which sum to target')


def output_first_incorrect_value(values: List[int], scan_length: int) -> None:
  print('Invalid value found: {}'.format(find_first_incorrect_value(values, scan_length)))


def output_contiguous_sum(values: List[int], scan_length: int) -> None:
  target = find_first_incorrect_value(values, scan_length)
  sum_range = find_contiguous_sum_to_value(values, target)
  max_v = max(sum_range)
  min_v = min(sum_range)
  print('Found range with max value {} and min value {}'.format(max_v, min_v))
  print('{} + {} = {}'.format(min_v, max_v, min_v + max_v))


output = {
  'first_incorrect_value': output_first_incorrect_value,
  'contiguous_sum': output_contiguous_sum
}


def main(resources: Resources) -> int:
  values = list(map(int, resources.read_resource(resources.config['numbers_file'])))
  
  output[resources.config['output']](values, resources.config['scan_length'])

  return 0
