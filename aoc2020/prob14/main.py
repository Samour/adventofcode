from typing import List, Dict
import re
from aoc2020.utils import Resources


MASK_BLANK = 'X'


class IBitMask:

    def parse_mask(self, mask: str) -> None:
        pass

    def apply_mask(self, value: int) -> List[int]:
        pass


class IRuntime:

    def get_bitmasks(self) -> List[IBitMask]:
        pass

    def set_maddr(self, addr: int, value: int) -> None:
        pass

    def get_memory(self) -> Dict[int, int]:
        pass

    def handle_instruction(self, instruction: str) -> None:
        pass


class IInstructionHandler:

    def try_instruction(self, runtime: IRuntime, instruction: str) -> bool:
        pass


class AbstractBitMask(IBitMask):

    def __init__(self, blank_symb: str):
        self._blank_symb = blank_symb
        self._mask = 0

    def parse_mask(self, mask: str) -> None:
        remask = ''
        for c in mask:
            if c == MASK_BLANK:
                remask += self._blank_symb
            else:
                remask += c

        self._mask = int(remask, 2)


class SetBitMask(AbstractBitMask):

    def __init__(self):
        super().__init__('0')

    def apply_mask(self, value: int) -> List[int]:
        return [value | self._mask]


class ClearBitMask(AbstractBitMask):

    def __init__(self):
        super().__init__('1')

    def apply_mask(self, value: int) -> List[int]:
        return [value & self._mask]


class FloatingBitMask(IBitMask):

    def __init__(self):
        self._mask = ''

    def parse_mask(self, mask: str) -> None:
        self._mask = mask

    def apply_mask(self, value: int) -> List[int]:
        values = [0]
        for i in range(len(self._mask)):
            c = self._mask[-i - 1]
            if c == '1':
                for j in range(len(values)):
                    values[j] |= 1 << i
            elif c == 'X':
                for j in range(len(values)):
                    values.append(values[j] | 1 << i)
            else:
                for j in range(len(values)):
                    values[j] |= value & (1 << i)

        return values


class Runtime(IRuntime):

    def __init__(self, bitmasks: List[IBitMask], handlers: List[IInstructionHandler]):
        self._bitmasks = bitmasks
        self._handlers = handlers
        self._mem: Dict[int, int] = {}

    def get_bitmasks(self) -> List[IBitMask]:
        return self._bitmasks

    def set_maddr(self, addr: int, value: int) -> None:
        self._mem[addr] = value

    def get_memory(self) -> Dict[int, int]:
        return self._mem

    def handle_instruction(self, instruction: str) -> None:
        for handler in self._handlers:
            if handler.try_instruction(self, instruction):
                return

        raise Exception('Could not understand instruction\n{}'.format(instruction))


class MaskInstructionHandler(IInstructionHandler):

    _EXPR = re.compile(r'mask\s*=\s*([0-1X]{36})')

    def try_instruction(self, runtime: IRuntime, instruction: str) -> bool:
        match = MaskInstructionHandler._EXPR.match(instruction)
        if not match:
            return False

        mask = match[1]
        for bitmask in runtime.get_bitmasks():
            bitmask.parse_mask(mask)
        
        return True


class MaskedValueInstructionHandler(IInstructionHandler):

    _EXPR = re.compile(r'mem\[([0-9]+)\]\s*=\s*([0-9]+)')

    def try_instruction(self, runtime: IRuntime, instruction: str) -> bool:
        match = MaskedValueInstructionHandler._EXPR.match(instruction)
        if not match:
            return False

        addr = int(match[1])
        value = int(match[2])
        for mask in runtime.get_bitmasks():
            value = mask.apply_mask(value)[0]
        runtime.set_maddr(addr, value)

        return True


class MaskedAddressInstructionHandler(IInstructionHandler):

    _EXPR = re.compile(r'mem\[([0-9]+)\]\s*=\s*([0-9]+)')

    def try_instruction(self, runtime: IRuntime, instruction: str) -> bool:
        match = MaskedAddressInstructionHandler._EXPR.match(instruction)
        if not match:
            return False

        addr = int(match[1])
        value = int(match[2])
        for mask in runtime.get_bitmasks():
            for v in mask.apply_mask(addr):
                runtime.set_maddr(v, value)

        return True


def runtime_factory(mode: str) -> IRuntime:
    if mode == 'value_mask':
        return Runtime(
            [
                SetBitMask(),
                ClearBitMask()
            ],
            [
                MaskInstructionHandler(),
                MaskedValueInstructionHandler()
            ]
        )
    elif mode == 'addr_mask':
        return Runtime(
            [FloatingBitMask()],
            [
                MaskInstructionHandler(),
                MaskedAddressInstructionHandler()
            ]
        )
    else:
        raise Exception('Unrecognized mode: {}'.format(mode))


def main(resources: Resources) -> int:
    text = resources.read_resource(resources.config['text'])
    runtime = runtime_factory(resources.config['mode'])
    for instruction in text:
        runtime.handle_instruction(instruction.strip())

    print(sum(runtime.get_memory().values()))

    return 0
