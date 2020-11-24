use crate::config::{Context, ContextFactory};
use serde::Deserialize;

#[derive(Deserialize)]
struct ProgramDefinition {
  name: String,
  text: String,
  subst: Option<Vec<Vec<usize>>>,
  target: Option<usize>,
  with_debug: Option<bool>,
}

#[derive(Deserialize)]
struct Config {
  programs: Vec<ProgramDefinition>,
}

#[derive(PartialEq, Eq, Debug)]
enum RuntimeState {
  EXECUTING,
  COMPLETED_SUCCESS,
  COMPLETED_ERROR,
}

struct Runtime {
  ic: usize,
  memory: Vec<usize>,
  completion_state: RuntimeState,
}

impl Runtime {
  fn new(text: Vec<usize>) -> Runtime {
    Runtime {
      ic: 0,
      memory: text,
      completion_state: RuntimeState::EXECUTING,
    }
  }

  fn tick(&mut self) {
    if self.completion_state != RuntimeState::EXECUTING {
      return;
    }

    match self.memory[self.ic] {
      1 => self.perform_add(),
      2 => self.perform_mult(),
      99 => self.perform_stop(),
      _ => self.panic(),
    }
  }

  fn perform_add(&mut self) {
    let src1_a = self.lafo(1);
    let src2_a = self.lafo(2);
    let dst_a = self.lafo(3);
    self.memory[dst_a] = self.memory[src1_a] + self.memory[src2_a];
    self.ic += 4;
  }

  fn perform_mult(&mut self) {
    let src1_a = self.lafo(1);
    let src2_a = self.lafo(2);
    let dst_a = self.lafo(3);
    self.memory[dst_a] = self.memory[src1_a] * self.memory[src2_a];
    self.ic += 4;
  }

  fn perform_stop(&mut self) {
    self.completion_state = RuntimeState::COMPLETED_SUCCESS;
  }

  fn panic(&mut self) {
    self.completion_state = RuntimeState::COMPLETED_ERROR;
  }

  fn lafo(&mut self, a: usize) -> usize {
    if self.ic + a >= self.memory.len() {
      self.panic();
      return 0;
    }

    let t_a = self.memory[self.ic + a];
    if t_a >= self.memory.len() {
      self.panic();
      return 0;
    }

    t_a
  }

  fn execute(&mut self) {
    while self.completion_state == RuntimeState::EXECUTING {
      self.tick()
    }
  }
}

fn load_program(text: &str, subst: &Vec<Vec<usize>>) -> Result<Runtime, String> {
  let mut instructions: Vec<usize> = Vec::new();
  for c in text.split(",") {
    let i = c.parse();
    if let Err(_) = i {
      return Err(format!("Could not parse instruction \"{}\"", c));
    }
    instructions.push(i.unwrap());
  }

  for sub in subst {
    instructions[sub[0]] = sub[1]
  }

  Ok(Runtime::new(instructions))
}

fn run_program(program: ProgramDefinition) -> Result<(), String> {
  let mut runtime = load_program(&program.text, &program.subst.unwrap_or_else(Vec::new))?;
  runtime.execute();

  println!("{}: {:?}", program.name, runtime.completion_state);
  println!("{}", memory_to_str(&runtime.memory));

  Ok(())
}

fn find_inputs(text: String, target: usize, with_debug: bool) -> Result<(), String> {
  for i in 0..100 {
    for j in 0..100 {
      if with_debug {
        println!("DEBUG: Evaluating inputs [{}, {}]", i, j);
      }
      let mut program = load_program(&text, &vec![
        vec![1, i],
        vec![2, j]
      ])?;
      program.execute();
      if with_debug {
        println!("DEBUG: Completed with state {:?}", program.completion_state);
        println!("DEBUG: Output = {}", program.memory[0]);
      }
      if program.completion_state == RuntimeState::COMPLETED_SUCCESS && program.memory[0] == target
      {
        println!("Inputs found: [{}, {}]", i, j);
        return Ok(());
      }
    }
  }

  println!("Could not find any set of inputs which match the output");
  Ok(())
}

fn memory_to_str(data: &Vec<usize>) -> String {
  data
    .iter()
    .map(|i| i.to_string())
    .collect::<Vec<String>>()
    .join(",")
}

pub fn main(context_factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = context_factory.create()?;
  for program in context.config.programs {
    if let Some(target) = program.target {
      find_inputs(program.text, target, program.with_debug.unwrap_or(false))?;
    } else {
      run_program(program)?;
    }
  }

  Ok(())
}
