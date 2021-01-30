from typing import Dict, List
from aoc2020.utils import Resources


JOLT_LEEWAY = 3


class DynamicAdaptorArrangement:

  def __init__(self, adaptors: List[int]):
    '''
    :param: adaptors Sorted list of adaptor jolts. Should include initial (0) value and target value.
    '''
    self._adaptors = adaptors
    self._memoized: Dict[int, int] = {}

  def _select_candidates(self, starting_from: int) -> List[int]:
    s_jolt = self._adaptors[starting_from]
    candidates: List[int] = []
    for i in range(starting_from + 1, len(self._adaptors)):
      if s_jolt + 3 >= self._adaptors[i]:
        candidates.append(i)
      else:
        break

    return candidates

  def _do_count(self, starting_from: int) -> int:
    # Recursive count without memoization
    if starting_from == len(self._adaptors) - 1:
      return 1 # Recursion escape

    count = 0
    for next_i in self._select_candidates(starting_from):
      count += self.count_arrangements(next_i)

    return count

  def count_arrangements(self, starting_from: int) -> int:
    '''
    Count no. of arrangements from the given starting index to end index

    :param: starting_from The index of list which is the first adaptor in chain
    '''

    if starting_from not in self._memoized:
      self._memoized[starting_from] = self._do_count(starting_from)
    return self._memoized[starting_from]


def count_variance_distributions(values: List[int]) -> None:
  diffs: Dict[int, int] = {}
  def add_diff(value: int) -> None:
    if value not in diffs:
      diffs[value] = 0
    diffs[value] += 1

  for i in range(1, len(values)):
    add_diff(values[i] - values[i - 1])

  print(diffs)
  print('{} * {} = {}'.format(diffs[1], diffs[3], diffs[1] * diffs[3]))


def count_arrangements(values: List[int]) -> None:
  adaptor_arrangements = DynamicAdaptorArrangement(values)
  count = adaptor_arrangements.count_arrangements(0)

  print('Number of possible arrangements: {}'.format(count))


output = {
  'count_variance_distributions': count_variance_distributions,
  'count_arrangements': count_arrangements
}


def main(resources: Resources) -> int:
  values = [ int(v) for v in resources.read_resource(resources.config['data']) ]
  values.insert(0, 0)
  values.sort()
  values.append(values[len(values) - 1] + 3)

  output[resources.config['output']](values)

  return 0
