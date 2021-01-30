from typing import List, Optional
from aoc2020.utils import Resources


PT_FLOOR = 'FLOOR'
PT_SEAT = 'SEAT'

C_FLOOR = '.'
C_SEAT_E = 'L'
C_SEAT_O = '#'


class Position:

  def __init__(self, p_type: str):
    self.type = p_type
    self.occupied = False
    self.next_state = False

  def tick(self) -> bool:
    change = self.occupied != self.next_state
    self.occupied = self.next_state
    return change


class ICountingStrategy:

  def count_adjacent(self, rows: List[List[Position]], i: int, j: int) -> int:
    pass


class AdjacentCountingStrategy(ICountingStrategy):

  def count_adjacent(self, rows: List[List[Position]], i: int, j: int) -> int:
    adjacent_c = 0
    for a_i in range(max([i - 1, 0]), min([ i + 2, len(rows[0])])):
      for a_j in range(max([j - 1, 0]), min([ j + 2, len(rows)])):
        if a_i == i and a_j == j:
          continue
        if rows[a_j][a_i].occupied:
          adjacent_c += 1

    return adjacent_c


class LineOfSightCountingStrategy(ICountingStrategy):

  def _get_seat_in_line(self, rows: List[List[Position]], i: int, j: int, d_i: int, d_j: int) -> Optional[Position]:
    while True:
      i += d_i
      j += d_j
      if i < 0 or i >= len(rows[0]) or j < 0 or j >= len(rows):
        return None
      elif rows[j][i].type == PT_SEAT:
        return rows[j][i]

  def count_adjacent(self, rows: List[List[Position]], i: int, j: int) -> int:
    count = 0
    for d_i in [-1, 0, 1]:
      for d_j in [-1, 0, 1]:
        if d_i == 0 and d_j == 0:
          continue
        pos = self._get_seat_in_line(rows, i, j, d_i, d_j)
        if pos is not None and pos.occupied:
          count += 1

    return count


class GameOfSeats:

  def __init__(self, counting_strategy: ICountingStrategy, max_seats: int, rows: List[List[Position]]):
    self._counting_strategy = counting_strategy
    self._max_seats = max_seats
    self._rows = rows

  def _count_adjacent(self, i: int, j: int) -> int:
    return self._counting_strategy.count_adjacent(self._rows, i, j)

  def _compute_cell(self, i: int, j: int) -> None:
    adjacent_c = self._count_adjacent(i, j)

    if adjacent_c == 0:
      self._rows[j][i].next_state = True
    elif adjacent_c >= self._max_seats:
      self._rows[j][i].next_state = False

  def _next_step(self) -> int:
    for i in range(len(self._rows[0])):
      for j in range(len(self._rows)):
        if self._rows[j][i].type == PT_SEAT:
          self._compute_cell(i, j)
    
    update_c = 0
    for i in range(len(self._rows[0])):
      for j in range(len(self._rows)):
        if self._rows[j][i].tick():
          update_c += 1

    return update_c

  def run(self) -> None:
    update_c = 1
    while update_c > 0:
      update_c = self._next_step()


counting_strategies = {
  'adjacent': AdjacentCountingStrategy,
  'line_of_sight': LineOfSightCountingStrategy
}


def game_factory(rows: List[List[Position]], game_params: dict) -> GameOfSeats:
  return GameOfSeats(counting_strategies[game_params['counting_strategy']](), game_params['max_seats'], rows)


def parse_map(lines: List[str]) -> List[List[Position]]:
  rows: List[List[Position]] = []
  for line in lines:
    row: List[Position] = []
    for c in line:
      p_type: str = PT_FLOOR
      if c == C_SEAT_E:
        p_type = PT_SEAT
      elif c != C_FLOOR:
        continue
      row.append(Position(p_type))
    if len(row):
      rows.append(row)

  return rows


def render_map(rows: List[List[Position]]) -> str:
  lines: List[str] = []
  for row in rows:
    line: List[str] = []
    for p in row:
      if p.type == PT_FLOOR:
        line.append(C_FLOOR)
      elif p.occupied:
        line.append(C_SEAT_O)
      else:
        line.append(C_SEAT_E)
    lines.append(''.join(line))
  
  return '\n'.join(lines)


def output_render_stable(rows: List[List[Position]], game_params: dict) -> None:
  seats = game_factory(rows, game_params)
  seats.run()
  print(render_map(rows))


def output_count_stable(rows: List[List[Position]], game_params: dict) -> None:
  seats = game_factory(rows, game_params)
  seats.run()

  occupied_c = 0
  for row in rows:
    for p in row:
      if p.occupied:
        occupied_c += 1
  print('Seats occupied: {}'.format(occupied_c))


output = {
  'render_stable': output_render_stable,
  'count_stable': output_count_stable
}


def main(resources: Resources) -> int:
  rows = parse_map(resources.read_resource(resources.config['map']))
  output[resources.config['output']](rows, resources.config['game_params'])

  return 0
