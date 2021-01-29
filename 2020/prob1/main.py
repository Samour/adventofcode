from typing import Optional, Iterable

class ValuesIterator:

  def __init__(self, values: list[int], select_c: int):
    self._values = values
    self._select_c = select_c
    self._idx = [ i for i in range(select_c) ]

  def _validate_idx(self) -> None:
    for idx in self._idx:
      if idx >= len(self._values):
        raise StopIteration()

  def _progress_idx(self, idx: Optional[int] = None) -> None:
    if idx is None:
      idx = self._select_c - 1
    self._idx[idx] += 1
    col_max = len(self._values) - (self._select_c - idx - 1)
    if self._idx[idx] < col_max or idx == 0:
      return
    self._progress_idx(idx - 1)
    for i in range(idx, self._select_c):
      self._idx[i] = self._idx[i - 1] + 1

  def __next__(self) -> list[int]:
    self._validate_idx()
    value = [ self._values[i] for i in self._idx ]
    self._progress_idx()

    return value


class ValuesIterable:

  def __init__(self, values: list[int], select_c: int):
    self._values = values
    self._select_c = select_c

  def __iter__(self) -> ValuesIterator:
    return ValuesIterator(self._values, self._select_c)


def mult(values: Iterable[int]) -> int:
  result = 1
  for v in values:
    result *= v

  return result


def main(config: dict) -> int:
  target = config['target']
  inputs = config['inputs']
  select_c = config['select_count']
  for values in ValuesIterable(inputs, select_c):
    if sum(values) == target:
      print('Found solution with values {}'.format(values))
      print('{} = {}'.format(' * '.join([ str(v) for v in values ]), mult(values)))
      return 0

  print('Could not find any solution')

  return 1
