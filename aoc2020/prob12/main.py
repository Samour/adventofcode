from typing import Dict, List, Optional, Tuple
import re
import math
from aoc2020.utils import Resources


TOTAL_DEG = 360
RAD_IN_DEG = 2 * math.pi / 360


class IPositionable:

  def get_waypoint(self) -> Optional['IPositionable']:
    pass

  def get_direction(self) -> int:
    pass

  def inc_direction(self, amount: int) -> None:
    pass

  def get_x(self) -> int:
    pass

  def inc_x(self, amount: int) -> None:
    pass

  def get_y(self) -> int:
    pass

  def inc_y(self, amount: int) -> None:
    pass

  def process_instruction(self, instruction: str) -> None:
    pass


class IPositionUpdateStrategy:

  def update_position(self, positionable: IPositionable, amount: int) -> None:
    pass


class Positionable(IPositionable):

  def __init__(self, position_updates: Dict[str, IPositionUpdateStrategy], waypoint: Optional[IPositionable], x: int,
      y: int, trace: bool):
    self._position_updates = position_updates
    self._waypoint = waypoint
    self._direction = 0
    self._x = x
    self._y = y
    self._trace = trace
    self._instruction_parser = re.compile(r'({})(\d+)'.format('|'.join(position_updates.keys())))

  def get_waypoint(self) -> Optional[IPositionable]:
    return self._waypoint

  def get_direction(self) -> int:
    return self._direction

  def inc_direction(self, amount: int) -> None:
    self._direction = (self._direction + amount) % TOTAL_DEG

  def get_x(self) -> int:
    return self._x

  def inc_x(self, amount: int) -> None:
    self._x += amount

  def get_y(self) -> int:
    return self._y

  def inc_y(self, amount: int) -> None:
    self._y += amount

  def process_instruction(self, instruction: str) -> None:
    parsed = self._instruction_parser.match(instruction)
    if not parsed:
      raise Exception('Could not parse instruction: {}'.format(instruction))
    self._position_updates[parsed[1]].update_position(self, int(parsed[2]))
    if self._trace:
      print(instruction.strip())
      print('{}, {} facing {}'.format(self._x, self._y, self._direction))
      if self._waypoint is not None:
        print('Waypoint at {}, {}'.format(self._waypoint.get_x(), self._waypoint.get_y()))
      print()


class LateralPositionUpdateStrategy(IPositionUpdateStrategy):

  def __init__(self, east_direction: bool):
    self._direction = 1 if east_direction else -1

  def update_position(self, positionable: IPositionable, amount: int) -> None:
    positionable.inc_x(amount * self._direction)


class LongitudinalPositionUpdateStrategy(IPositionUpdateStrategy):

  def __init__(self, north_direction: bool):
    self._direction = 1 if north_direction else -1

  def update_position(self, positionable: IPositionable, amount: int) -> None:
    positionable.inc_y(amount * self._direction)


class ForwardPositionUpdateStrategy(IPositionUpdateStrategy):

  def update_position(self, positionable: IPositionable, amount: int) -> None:
    positionable.inc_x(round(amount * math.cos(positionable.get_direction() * RAD_IN_DEG)))
    positionable.inc_y(round(amount * math.sin(positionable.get_direction() * RAD_IN_DEG)))


class RotatePositionUpdateStrategy(IPositionUpdateStrategy):

  def __init__(self, turn_left: bool):
    self._direction = 1 if turn_left else -1

  def update_position(self, positionable: IPositionable, amount: int) -> None:
    positionable.inc_direction(amount * self._direction)


class RotateWaypointUpdateStrategy(IPositionUpdateStrategy):

  def __init__(self, is_anticlockwise: bool):
    self._direction = 1 if is_anticlockwise else -1
  
  def update_position(self, positionable: IPositionable, amount: int) -> None:
    waypoint = positionable.get_waypoint()
    if waypoint is None:
      raise Exception('Cannot rotate waypoint as it does not exist')
    dist = math.sqrt(waypoint.get_x() ** 2 + waypoint.get_y() ** 2)
    current_th = math.atan2(waypoint.get_y(), waypoint.get_x())
    new_th = current_th + amount * self._direction * RAD_IN_DEG
    r_x = round(dist * math.cos(new_th))
    r_y = round(dist * math.sin(new_th))
    waypoint.inc_x(r_x - waypoint.get_x())
    waypoint.inc_y(r_y - waypoint.get_y())


class TowardWaypointUpdateStategy(IPositionUpdateStrategy):
  
  def update_position(self, positionable: IPositionable, amount: int) -> None:
    waypoint = positionable.get_waypoint()
    if waypoint is None:
      raise Exception('Cannot rotate waypoint as it does not exist')
    for i in range(amount):
      positionable.inc_x(waypoint.get_x())
      positionable.inc_y(waypoint.get_y())


class ForwardToWaypointUpdateStrategy(IPositionUpdateStrategy):

  def __init__(self, instruction: str):
    self._instruction = instruction

  def update_position(self, positionable: IPositionable, amount: int) -> None:
    waypoint = positionable.get_waypoint()
    if waypoint is None:
      raise Exception('Cannot forward instruction as waypoint does not exist')
    waypoint.process_instruction('{}{}'.format(self._instruction, amount))


def simple_position_updates() -> Dict[str, IPositionUpdateStrategy]:
  return {
    'N': LongitudinalPositionUpdateStrategy(True),
    'S': LongitudinalPositionUpdateStrategy(False),
    'E': LateralPositionUpdateStrategy(True),
    'W': LateralPositionUpdateStrategy(False),
    'L': RotatePositionUpdateStrategy(True),
    'R': RotatePositionUpdateStrategy(False),
    'F': ForwardPositionUpdateStrategy()
  }


def waypoint_position_updates() -> Tuple[Dict[str, IPositionUpdateStrategy], Dict[str, IPositionUpdateStrategy]]:
  waypoint_updates = {
    'N': LongitudinalPositionUpdateStrategy(True),
    'S': LongitudinalPositionUpdateStrategy(False),
    'E': LateralPositionUpdateStrategy(True),
    'W': LateralPositionUpdateStrategy(False)
  }
  positionable_updates = {
    'N': ForwardToWaypointUpdateStrategy('N'),
    'S': ForwardToWaypointUpdateStrategy('S'),
    'E': ForwardToWaypointUpdateStrategy('E'),
    'W': ForwardToWaypointUpdateStrategy('W'),
    'L': RotateWaypointUpdateStrategy(True),
    'R': RotateWaypointUpdateStrategy(False),
    'F': TowardWaypointUpdateStategy()
  }

  return positionable_updates, waypoint_updates


def create_positionable(strategy: str, waypoint_opts: Optional[dict], trace: bool) -> Positionable:
  if strategy == 'simple':
    position_updates = simple_position_updates()
    return Positionable(position_updates, None, 0, 0, trace)
  elif strategy == 'waypoint':
    if waypoint_opts is None:
      raise Exception('Strategy \'{}\' selected, but waypoint configuration not provided'.format(strategy))
    position_updates, waypoint_updates = waypoint_position_updates()
    waypoint = Positionable(waypoint_updates, None, waypoint_opts['init_x'], waypoint_opts['init_y'], False)
    return Positionable(position_updates, waypoint, 0, 0, trace)
  else:
    raise Exception('Unrecognised strategy: {}'.format(strategy))


def execute_instructions(instructions: List[str], strategy: str, waypoint_opts: Optional[dict], trace: bool) -> Positionable:
  positionable = create_positionable(strategy, waypoint_opts, trace)
  for i in instructions:
    positionable.process_instruction(i)

  return positionable


def output_manhatten(positionable: Positionable) -> None:
  print('Final position: {}, {}'.format(positionable.get_x(), positionable.get_y()))
  print('Manhatten distance: {}'.format(abs(positionable.get_x()) + abs(positionable.get_y())))


def main(resources: Resources) -> int:
  instructions = resources.read_resource(resources.config['instructions'])
  positionable = execute_instructions(
    instructions,
    resources.config['strategy'],
    resources.config.get('waypoint'),
    resources.config.get('trace', False)
  )
  output_manhatten(positionable)

  return 0
