from typing import Optional
import re
from utils import Resources


FIELD_SEP = ' '
KV_SEP = ':'


class PassportData:

  def __init__(self):
    self.data = {}

  def parse(self, data: str) -> None:
    fields = data.split(FIELD_SEP)
    for field in fields:
      key_value = field.split(KV_SEP)
      self.data[key_value[0].strip()] = key_value[1].strip()


class BatchParser:

  def __init__(self, src_data: list[str]):
    self._src_data = src_data
    self._i = 0

  def _get_line(self) -> str:
    return self._src_data[self._i].strip()

  def get_next_passport(self) -> Optional[PassportData]:
    if self._i >= len(self._src_data):
      return None

    passport = PassportData()
    while not len(passport.data) or len(self._get_line()):
      if len(self._get_line()):
        passport.parse(self._get_line())
      self._i += 1
      if self._i >= len(self._src_data):
        break

    return passport


class IFieldValidator:

  def get_field_name(self) -> str:
    pass

  def validate_value(self, value: str) -> bool:
    pass


class RangeValidator(IFieldValidator):

  def __init__(self, name: str, min_c: int, max_c: int):
    self._name = name
    self._min = min_c
    self._max = max_c

  def get_field_name(self) -> str:
    return self._name

  def validate_value(self, value: str) -> bool:
    try:
      i_value = int(value)
      return i_value >= self._min and i_value <= self._max
    except:
      return False


class TypedRangeValidator(IFieldValidator):

  def __init__(self, name: str, type_ranges: dict[str, RangeValidator]):
    self._name = name
    self._type_ranges = type_ranges
    self._regex = re.compile(r'(\d+)(\w+)')

  def get_field_name(self) -> str:
    return self._name

  def validate_value(self, value: str) -> bool:
    match = self._regex.match(value)
    if not match:
      return False
    if match[2] not in self._type_ranges:
      return False
    
    return self._type_ranges[match[2]].validate_value(match[1])


class RegexValidator(IFieldValidator):

  def __init__(self, name: str, exp: str):
    self._name = name
    self._regex = re.compile(exp)

  def get_field_name(self) -> str:
    return self._name

  def validate_value(self, value: str) -> bool:
    return self._regex.match(value) is not None


class PassportValidator:

  def __init__(self, required_fields: list[IFieldValidator], debug: bool):
    self._required_fields = required_fields
    self._debug = debug

  def validate(self, passport: PassportData) -> bool:
    for field in self._required_fields:
      name = field.get_field_name()
      if name not in passport.data:
        if self._debug:
          print('Validation failed due to missing field: {}'.format(name))
        return False
      if not field.validate_value(passport.data[name]):
        if self._debug:
          print('Validation of field \'{}\' failed due to {}'.format(field.get_field_name(), field.__class__.__name__))
        return False

    return True


def parse_range_validator(name: str, params: dict) -> RangeValidator:
  return RangeValidator(name, params['min'], params['max'])


def parse_typed_range_validator(name: str, params: dict) -> TypedRangeValidator:
  return TypedRangeValidator(name, {
    t: RangeValidator('{}_{}'.format(name, t), params[t]['min'], params[t]['max']) for t in params
  })


def parse_regex_validator(name: str, params: dict) -> RegexValidator:
  return RegexValidator(name, params['exp'])


validator_providers = {
  'RangeValidator': parse_range_validator,
  'TypedRangeValidator': parse_typed_range_validator,
  'RegexValidator': parse_regex_validator
}


def parse_field(details: dict) -> IFieldValidator:
  if details['validator']['impl'] not in validator_providers:
    raise Exception('Could not identity validator: {}'.format(details['validator']['impl']))

  return validator_providers[details['validator']['impl']](details['name'], details['validator']['params'])


def main(resources: Resources) -> int:
  debug = resources.config.get('debug', False)
  validator = PassportValidator([ parse_field(f) for f in resources.config['required_fields'] ], debug)
  batch_parser = BatchParser(resources.read_resource(resources.config['data']))

  valid_c = 0
  passport = batch_parser.get_next_passport()
  while passport is not None:
    if validator.validate(passport):
      valid_c += 1
    passport = batch_parser.get_next_passport()

  print('No of valid passports: {}'.format(valid_c))

  return 0
