from typing import Optional, Any, List
from aoc2020.utils import Resources


class GroupAnswers:

  def __init__(self, answers: List[str], debug: bool):
    self._answers = answers
    self._debug = debug

  def _log(self, msg: Any) -> None:
    if self._debug:
      print(msg)

  def get_distinct_positive_answers(self):
    dpa = set()
    for answ in self._answers:
      for a in answ:
        dpa.add(a)

    return len(dpa)

  def get_unanimous_positive_answers(self):
    self._log('Starting unanimous check for group')
    self._log(self._answers)
    dpa = set(self._answers[0])
    for answ in self._answers:
      self._log('dpa: {}'.format(dpa))
      self._log('answ: {}'.format(answ))
      for a in list(dpa):
        if a not in answ:
          dpa.remove(a)

    return len(dpa)


class GroupFeeder:

  def __init__(self, data: List[str], debug: bool):
    self._data = data
    self._i = 0
    self._debug = debug

  def _get_line(self) -> str:
    return self._data[self._i].strip()

  def get_next_group(self) -> Optional[GroupAnswers]:
    if self._i >= len(self._data):
      return None
    answers: List[str] = []
    while not len(answers) or len(self._get_line()):
      if len(self._get_line()):
        answers.append(self._get_line())
      self._i += 1
      if self._i >= len(self._data):
        break

    if len(answers):
      return GroupAnswers(answers, self._debug)
    else:
      return None


analysis_strategies = {
  'any': lambda g: g.get_distinct_positive_answers(),
  'all': lambda g: g.get_unanimous_positive_answers()
}


def main(resources: Resources) -> int:
  group_feeder = GroupFeeder(resources.read_resource(resources.config['data']), resources.config.get('debug', False))
  check_type = analysis_strategies[resources.config['check_type']]

  total_a = 0
  group = group_feeder.get_next_group()
  while group is not None:
    total_a += check_type(group)
    group = group_feeder.get_next_group()

  print('Total answers across groups: {}'.format(total_a))

  return 0
