from typing import Optional
from utils import Resources


class IInterpreter:

  def get_acc(self) -> int:
    pass

  def inc_acc(self, value: int) -> None:
    pass

  def get_ip(self) -> int:
    pass

  def inc_ip(self, value: int) -> None:
    pass

  def run(self) -> None:
    pass

  def interrupt(self) -> None:
    pass

  def is_interrupted(self) -> bool:
    pass


class IInstruction:

  def execute(self, runtime: IInterpreter, arg: int) -> None:
    pass


class IRuntimeObserver:

  def post_instruction(self, runtime: IInterpreter, instruction: str) -> None:
    pass


class AccInstruction(IInstruction):

  def execute(self, runtime: IInterpreter, arg: int) -> None:
    runtime.inc_acc(arg)
    runtime.inc_ip(1)


class JumpInstruction(IInstruction):

  def execute(self, runtime: IInterpreter, arg: int) -> None:
    runtime.inc_ip(arg)


class NoopInstruction(IInstruction):
  
  def execute(self, runtime: IInterpreter, arg: int) -> None:
    runtime.inc_ip(1)


def parse_instruction(instruction: str) -> tuple[str, int]:
  parts = instruction.split(' ')
  return parts[0].strip(), int(parts[1])


class InterpreterRuntime(IInterpreter):

  def __init__(self, instruction_set: dict[str, IInstruction], observer: Optional[IRuntimeObserver], text: list[str]):
    self._instruction_set = instruction_set
    self._observer = observer
    self._text = text
    self._interrupted = False
    self.accumulator = 0
    self.ip = 0

  def get_acc(self) -> int:
    return self.accumulator

  def inc_acc(self, value: int) -> None:
    self.accumulator += value

  def get_ip(self) -> int:
    return self.ip

  def inc_ip(self, value: int) -> None:
    self.ip += value

  def _ip_valid(self) -> bool:
    return self.ip >= 0 and self.ip < len(self._text)

  def _execute_instruction(self) -> None:
    command = self._text[self.ip]
    opcode, arg = parse_instruction(command)
    if opcode not in self._instruction_set:
      raise Exception('Instruction not recognized: {}'.format(opcode))

    self._instruction_set[opcode].execute(self, arg)
    if self._observer:
      self._observer.post_instruction(self, command)

  def run(self) -> None:
    while not self._interrupted and self._ip_valid():
      self._execute_instruction()

  def interrupt(self) -> None:
    self._interrupted = True

  def is_interrupted(self) -> bool:
    return self._interrupted


instruction_set = {
  'acc': AccInstruction(),
  'jmp': JumpInstruction(),
  'nop': NoopInstruction()
}


class ChainObserver(IRuntimeObserver):

  def __init__(self, observers: list[IRuntimeObserver]):
    self._observers = observers

  def post_instruction(self, runtime: IInterpreter, instruction: str) -> None:
    for observer in self._observers:
      observer.post_instruction(runtime, instruction)


class TracingObserver(IRuntimeObserver):

  def __init__(self):
    self._step = 0

  def post_instruction(self, runtime: IInterpreter, instruction: str) -> None:
    print('Step {}'.format(self._step))
    print(instruction.strip())
    print('acc: {}     ip: {}'.format(runtime.get_acc(), runtime.get_ip()))
    print()

    self._step += 1


class RecursionObserver(IRuntimeObserver):

  def __init__(self, silent: bool):
    self._encountered_positions = set()
    self._silent = silent

  def post_instruction(self, runtime: IInterpreter, instruction: str) -> None:
    if runtime.get_ip() in self._encountered_positions:
      if not self._silent:
        print('Position {} re-encountered'.format(runtime.get_ip()))
      runtime.interrupt()
    else:
      self._encountered_positions.add(runtime.get_ip())


def observer_factory(name: str) -> Optional[IRuntimeObserver]:
  if name == 'tracing':
    return TracingObserver()
  elif name == 'recursion':
    return RecursionObserver(False)
  elif name == 'recursion::silent':
    return RecursionObserver(True)
  else:
    return None


def build_observers(names: list[str]) -> Optional[IRuntimeObserver]:
  if not names or len(names) == 0:
    return None
  elif len(names) == 1:
    return observer_factory(names[0])

  observers = list(filter(
    lambda o: o is not None,
    map(observer_factory, names)
  ))
  return ChainObserver(observers)


def execute_instructions(instructions: list[str], observers: Optional[list[str]]) -> IInterpreter:
  observer = build_observers(observers)
  runtime = InterpreterRuntime(instruction_set, observer, instructions)

  runtime.run()
  return runtime


def run_until_recursion(instructions: list[str], observers: Optional[list[str]]) -> None:
  runtime = execute_instructions(instructions, observers)
  print('Accumulator value: {}'.format(runtime.get_acc()))


def flip_instruction(instruction: str) -> Optional[str]:
  translations = {
    'nop': 'jmp',
    'jmp': 'nop'
  }
  opcode, arg = parse_instruction(instruction)
  if opcode not in translations:
    return None
  
  return '{} {}'.format(translations[opcode], arg)


def gen_alt_text(instructions: list[str], target: int) -> Optional[list[str]]:
  alt_instr = flip_instruction(instructions[target])
  if alt_instr is None:
    return None

  alt_text = []
  for i, instruction in enumerate(instructions):
    if i == target:
      alt_text.append(alt_instr)
    else:
      alt_text.append(instruction)

  return alt_text


def permute_instructions(instructions: list[str], observers: Optional[list[str]]) -> None:
  for i in range(len(instructions)):
    alt_text = gen_alt_text(instructions, i)
    if alt_text is None:
      continue

    runtime = execute_instructions(alt_text, observers)
    if not runtime.is_interrupted() and runtime.get_ip() == len(instructions):
      print('Corrected program text. Accumulator value is {}'.format(runtime.get_acc()))
      return

  print('Could not correct program text')


analyses = {
  'run_until_recursion': run_until_recursion,
  'permute_instructions': permute_instructions
}


def main(resources: Resources) -> int:
  instructions = list(filter(
    lambda l: len(l),
    map(
      lambda l: l.strip(),
      resources.read_resource(resources.config['instructions'])
    )
  ))

  analyses[resources.config['analysis']](instructions, resources.config.get('observers'))
