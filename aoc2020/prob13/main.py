from typing import List, Callable
import math
from aoc2020.utils import Resources


class BusData:

    def __init__(self, l_time: int):
        self.l_time = l_time
        self.bus_ids: List[int] = []


class TimingElement:

    def __init__(self):
        self.value = 1
        self._inc = 1

    def push_constraint(self, base: int, offset: int) -> None:
        print('value: {}'.format(self.value))
        print('inc: {}'.format(self._inc))
        while -self.value % base != offset % base:
            self.value += self._inc

        self._inc = (self._inc * base) // gcd(self._inc, base)


def gcd(a: int, b: int) -> int:
    while b > 0:
        t = b
        b = a % b
        a = t

    return a


def bus_wait_time(l_time: int, bus_id: int) -> int:
    return -l_time % bus_id


def choose_earliest_bus(bus_data: BusData) -> None:
    l_bus_time = -1
    l_bus_id = -1
    for v in bus_data.bus_ids:
        if v == -1:
            continue
        if l_bus_id == -1 or bus_wait_time(bus_data.l_time, v) < l_bus_time:
            l_bus_time = bus_wait_time(bus_data.l_time, v)
            l_bus_id = v

    print(l_bus_id * bus_wait_time(bus_data.l_time, l_bus_id))


def find_c_time(bus_data: BusData) -> None:
    element = TimingElement()
    for i, v in enumerate(bus_data.bus_ids):
        if v <= 0:
            continue
        print('push({}, {})'.format(v, i))
        element.push_constraint(v, i)

    print(element.value)


def parse_file(lines: List[str]) -> BusData:
    bus_data = BusData(int(lines[0]))
    for v in lines[1].split(','):
        try:
            bus_data.bus_ids.append(int(v))
        except:
            bus_data.bus_ids.append(-1)

    return bus_data


def main(resources: Resources) -> int:
    bus_data = parse_file(resources.read_resource(resources.config['data']))

    if resources.config['action'] == 'l_time':
        choose_earliest_bus(bus_data)
    elif resources.config['action'] == 'c_times':
        find_c_time(bus_data)

    return 0
