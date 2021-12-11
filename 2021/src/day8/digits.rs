use std::collections::HashSet;

pub struct DigitalOutput {
  pub segments: HashSet<char>,
}

impl DigitalOutput {
  fn new(segments: HashSet<char>) -> DigitalOutput {
    DigitalOutput { segments }
  }
}

pub struct DisplayAnalysis {
  pub sample_outputs: Vec<DigitalOutput>,
  pub final_output: [DigitalOutput; 4],
}

impl DisplayAnalysis {
  fn new(sample_outputs: Vec<DigitalOutput>, final_output: [DigitalOutput; 4]) -> DisplayAnalysis {
    DisplayAnalysis {
      sample_outputs,
      final_output,
    }
  }
}

fn parse_digital_output(raw_output: &str) -> Option<DigitalOutput> {
  let raw_output = raw_output.trim();
  if raw_output.len() > 0 {
    Some(DigitalOutput::new(raw_output.chars().collect()))
  } else {
    None
  }
}

fn parse_display_analysis(raw_line: &str) -> Option<DisplayAnalysis> {
  let line: Vec<&str> = raw_line.split("|").collect();
  if line.len() != 2 {
    return None;
  }
  let sample_outputs: Vec<DigitalOutput> = line[0]
    .split(" ")
    .map(parse_digital_output)
    .filter(|d| d.is_some())
    .map(|d| d.unwrap())
    .collect();
  let mut final_output: Vec<DigitalOutput> = line[1]
    .split(" ")
    .map(parse_digital_output)
    .filter(|d| d.is_some())
    .map(|d| d.unwrap())
    .collect();
  if final_output.len() != 4 {
    return None;
  }

  let o4 = final_output.pop().unwrap();
  let o3 = final_output.pop().unwrap();
  let o2 = final_output.pop().unwrap();
  let o1 = final_output.pop().unwrap();
  Some(DisplayAnalysis::new(sample_outputs, [o1, o2, o3, o4]))
}

pub fn parse_displays(raw_data: &str) -> Vec<DisplayAnalysis> {
  raw_data
    .split("\n")
    .map(parse_display_analysis)
    .filter(|d| d.is_some())
    .map(|d| d.unwrap())
    .collect()
}
