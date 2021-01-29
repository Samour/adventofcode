from utils import Resources


HIGH_CHAR = {'B', 'R'}


class SeatIdentifier:

  def __init__(self, src: str):
    self.src = src

  def get_seat_id(self) -> int:
    b_val = ''
    for c in self.src:
      if c in HIGH_CHAR:
        b_val += '1'
      else:
        b_val += '0'
    
    return int(b_val, 2)


def output_all(identifiers: list[SeatIdentifier]) -> None:
  for identifier in identifiers:
    print('{}: {}'.format(identifier.src, identifier.get_seat_id()))


def output_max(identifiers: list[SeatIdentifier]) -> None:
  max_id = max(map(lambda i: i.get_seat_id(), identifiers))
  print('Highest seat ID: {}'.format(max_id))


def output_missing(identifiers: list[SeatIdentifier]) -> None:
  all_ids = { i.get_seat_id() for i in identifiers }
  for i in range(min(all_ids), max(all_ids)):
    if i not in all_ids:
      print('Missing seat no: {}'.format(i))
      return

  print('Could not find missing seat ID!')


action_map = {
  'all': output_all,
  'max': output_max,
  'missing': output_missing
}


def main(resources: Resources) -> int:
  identifiers = [ SeatIdentifier(s.strip()) for s in resources.read_resource(resources.config['data']) ]

  action_map[resources.config['output']](identifiers)

  return 0
