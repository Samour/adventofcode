import re


policy_format = re.compile(r'(\d+)-(\d+) (\w)')


class IPasswordPolicy:

  def validate(self, password: str) -> bool:
    pass


class PasswordCountPolicy(IPasswordPolicy):

  def __init__(self, subject: str, min_c: int, max_c: int):
    self.subject = subject
    self.min = min_c
    self.max = max_c

  def validate(self, password: str) -> bool:
    count = 0
    for c in password:
      if c == self.subject:
        count += 1

    return count >= self.min and count <= self.max


class PasswordPositionPolicy(IPasswordPolicy):
  
  def __init__(self, subject: str, low: int, high: int):
    self.subject = subject
    self.low = low
    self.high = high

  def validate(self, password: str) -> bool:
    return (password[self.low - 1] == self.subject) != (password[self.high - 1] == self.subject)


class PasswordEntry:

  def __init__(self, policy: IPasswordPolicy, password: str):
    self.policy = policy
    self.password = password

  def validate(self) -> bool:
    return self.policy.validate(self.password)


def policy_selector(policy_type: str, subject: str, low: int, high: str) -> IPasswordPolicy:
  if policy_type == 'PasswordCountPolicy':
    return PasswordCountPolicy(subject, low, high)
  elif policy_type == 'PasswordPositionPolicy':
    return PasswordPositionPolicy(subject, low, high)
  else:
    raise Exception('Policy type not recognized: {}'.format(policy_type))


def parse_policy(policy_type: str, policy: str) -> IPasswordPolicy:
  res = policy_format.match(policy)
  return policy_selector(policy_type, res[3], int(res[1]), int(res[2]))


def parse_entry(policy_type: str, entry: str) -> PasswordEntry:
  parts = entry.split(':')
  policy = parse_policy(policy_type, parts[0].strip())

  return PasswordEntry(policy, parts[1].strip())


def main(config: dict) -> int:
  policy_type = config['policy_type']
  valid_c = 0
  debug = config.get('debug', False)
  for password in config['passwords']:
    entry = parse_entry(policy_type, password)
    valid = entry.validate()
    if valid:
      valid_c += 1
    if debug:
      print('Entry `{}` is valid: {}'.format(password, valid))

  print('Number of valid passwords: {}'.format(valid_c))

  return 0
