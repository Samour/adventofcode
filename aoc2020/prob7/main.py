import re
from typing import cast, Match, Optional, List, Dict, Set
from aoc2020.utils import Resources


content_regex = re.compile(r'^([\w ]+) bags contain (\d+ [\w ]+ bags?(, \d+ [\w ]+ bags?)*).')
constraint_regex = re.compile(r' ?(\d+) ([\w ]+) bags?')
no_content_regex = re.compile(r'^([\w ]+) bags contain no other bags.')


class BagCount:

  def __init__(self, colour: str, count: int):
    self.colour = colour
    self.count = count


class ContentsRule:

  def __init__(self, colour: str, counts: List[BagCount]):
    self.colour = colour
    self.counts = counts

  def count_bags_inside(self, bag_graph: Dict[str, 'ContentsRule']) -> int:
    bag_c = 0
    for count in self.counts:
      bag_c += (1 + bag_graph[count.colour].count_bags_inside(bag_graph)) * count.count

    return bag_c


class BagGraph:

  def __init__(self):
    self.colours: Dict[str, ContentsRule] = {}
    self.contained_by: Dict[str, Set[str]] = {}

  def _add_reverse(self, container: str, containing: str) -> None:
    if containing not in self.contained_by:
      self.contained_by[containing] = set()
    self.contained_by[containing].add(container)

  def add_to_graph(self, rule: ContentsRule) -> None:
    self.colours[rule.colour] = rule
    for count in rule.counts:
      self._add_reverse(rule.colour, count.colour)

  def count_bags_in_target(self, target: str) -> int:
    return self.colours[target].count_bags_inside(self.colours)


def parse_content_rule(match: Match) -> ContentsRule:
  bag_parts = match[2].split(',')
  return ContentsRule(
    match[1].strip(),
    list(map(
      lambda m: BagCount(m[2].strip(), int(m[1])),
      map(
        lambda c: cast(re.Match[str], constraint_regex.match(c)),
        bag_parts
      )
    ))
  )


def parse_rule(rule: str) -> Optional[ContentsRule]:
  match = content_regex.match(rule)
  if match is not None:
    return parse_content_rule(match)
  match = no_content_regex.match(rule)
  if match is not None:
    return ContentsRule(match[1].strip(), [])
  
  return None


def build_graph(rules: List[ContentsRule]) -> BagGraph:
  graph = BagGraph()
  for rule in rules:
    graph.add_to_graph(rule)

  return graph


def output_all_colours(rules: List[ContentsRule], params: dict) -> None:
  colours = set()
  for rule in rules:
    colours.add(rule.colour)
    for count in rule.counts:
      colours.add(count.colour)
  print('Colours referenced in rules:')
  for c in colours:
    print(c)


def output_possible_containers(rules: List[ContentsRule], params: dict) -> None:
  graph = build_graph(rules)
  containers = set()
  get_containers = [params['target']]
  while len(get_containers):
    target = get_containers.pop()
    for container in graph.contained_by.get(target, set()):
      containers.add(container)
      get_containers.append(container)

  print(len(containers))


def output_count_containing(rules: List[ContentsRule], params: dict) -> None:
  graph = build_graph(rules)
  target = params['target']
  count = graph.count_bags_in_target(target)
  print('Number of bags inside {}: {}'.format(target, count))


output_strategy = {
  'colours': output_all_colours,
  'count_possible_containers': output_possible_containers,
  'count_containing': output_count_containing
}


def main(resources: Resources) -> int:
  rules = list(map(
    lambda r: cast(ContentsRule, r),
    filter(
      lambda r: r is not None,
      map(parse_rule, resources.read_resource(resources.config['data']))
    )
  ))

  output_strategy[resources.config['output']](rules, resources.config.get('params', {}))

  return 0
