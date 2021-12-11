//!
//! Algorithm:
//!
//! 1. Define a canonical mapping of segments where each segment is known
//! 2. Compute a frequency chart for each canonical segment based on how many digits that segment appears in
//! 3. Define a mapping of scrambled segment IDs to candidate canonical segment IDs. The initial state of
//! each mapping is all canonical segments with the same frequency as the scrambled segment.
//! 4. For each sample value in the source data, attempt to determine its numeric value as follows:
//! 4a. Start with candidate values = [0, 9]
//! 4b. Filter candidate values where the number of segments in candidate = number of segments in sample
//! 4c. For each segment where the scrambled -> canonical mapping is known (1 candidate):
//! 4c)i. If the segment is part of the sample, remove any candidate value which does not include that segment
//! 4c)ii. If the segment is not part of the sample, remove any candidate value which does include that segment
//! 5. For each remaining candidate value after this filtering is applied, aggregate all canonical segment IDs into a
//! single set
//! 6. For each scrambled segment in the sample value, update its canonical mapping by removing any segment which is
//! not contained in the set from step 4
//! 7. If there are remaining ambiguous segment mappings, repeat from step 4 until all mappings are found
//! 8. To find the value of a DigitalOutput, map the segments using the scrambled -> canonical segment mappings. Then
//! find the value of that canonical DigitalOutput using the mapping from step 1.

use std::collections::{HashMap, HashSet};

use crate::day8::digits::{DigitalOutput, DisplayAnalysis};

const MAX_LOOP: i32 = 1_000;
const ALG_FEATURE_FREQUENCY_ANALYSIS: &str = "FREQUENCY_ANALYSIS";
const ALG_FEATURE_SEGMENT_COUNT: &str = "SEGMENT_COUNT";
const ALG_FEATURE_OVERLAP_MAPPINGS: &str = "OVERLAP_MAPPINGS";
const ALG_FEATURE_SUBSET_CANDIDATES: &str = "SUBSET_CANDIDATES";

fn initialise_segments() -> HashSet<char> {
  let mut segments: HashSet<char> = HashSet::new();
  for c in 'a'..'h' {
    segments.insert(c);
  }

  segments
}

fn initialise_values() -> HashSet<usize> {
  let mut values: HashSet<usize> = HashSet::new();
  for i in 0..10 {
    values.insert(i);
  }

  values
}

pub struct Solution {
  final_output: [DigitalOutput; 4],
  scrambled_digit_mappings: HashMap<String, usize>,
}

impl Solution {
  fn new(
    final_output: [DigitalOutput; 4],
    scrambled_digit_mappings: HashMap<String, usize>,
  ) -> Solution {
    Solution {
      final_output,
      scrambled_digit_mappings,
    }
  }

  pub fn extract_value(&self) -> Result<i32, String> {
    let mut value: i32 = 0;
    for digit in &self.final_output {
      let dig_value: i32 = self
        .scrambled_digit_mappings
        .get(&digit.canonical)
        .ok_or(String::from("Value not found in solution"))?
        .clone() as i32;
      value *= 10;
      value += dig_value;
    }

    Ok(value)
  }
}

fn compute_segment_frequencies(digits: Vec<&DigitalOutput>) -> HashMap<char, usize> {
  let mut result: HashMap<char, usize> = HashMap::new();
  for digit in digits {
    for &segment in &digit.segments {
      let mut count = result.remove(&segment).unwrap_or(0);
      count += 1;
      result.insert(segment, count);
    }
  }

  result
}

fn compute_segments_by_frequency(digits: Vec<&DigitalOutput>) -> HashMap<usize, HashSet<char>> {
  let mut result: HashMap<usize, HashSet<char>> = HashMap::new();
  for (segment, frequency) in compute_segment_frequencies(digits) {
    let mut segments = result.remove(&frequency).unwrap_or_else(HashSet::new);
    segments.insert(segment);
    result.insert(frequency, segments);
  }

  result
}

struct SolverInternal<'a> {
  canonical_digits: &'a HashMap<usize, DigitalOutput>,
  segment_candidate_mappings: HashMap<char, HashSet<char>>,
  disabled_features: &'a HashSet<String>,
}

impl SolverInternal<'_> {
  fn new<'a>(
    canonical_digits: &'a HashMap<usize, DigitalOutput>,
    disabled_features: &'a HashSet<String>,
  ) -> SolverInternal<'a> {
    let mut segment_candidate_mappings: HashMap<char, HashSet<char>> = HashMap::new();
    for c in initialise_segments() {
      segment_candidate_mappings.insert(c, initialise_segments());
    }
    SolverInternal {
      canonical_digits,
      segment_candidate_mappings,
      disabled_features,
    }
  }

  fn reduce_segments_by_frequency(&mut self, digits: Vec<&DigitalOutput>) -> Result<(), String> {
    let canonical_frequencies =
      compute_segments_by_frequency(self.canonical_digits.values().collect());
    let scrambled_frequencies = compute_segment_frequencies(digits);
    for (scrambled, frequency) in scrambled_frequencies {
      let canonical = canonical_frequencies
        .get(&frequency)
        .ok_or(String::from("Could not find segment with this frequency"))?;
      let mapped_segments = self
        .segment_candidate_mappings
        .remove(&scrambled)
        .ok_or(String::from("Segment missing from mapping"))?;
      let mut updated = mapped_segments.clone();
      for s in mapped_segments {
        if !canonical.contains(&s) {
          updated.remove(&s);
        }
      }
      self.segment_candidate_mappings.insert(scrambled, updated);
    }

    Ok(())
  }

  fn get_canonical_digit(&self, value: usize) -> Result<&DigitalOutput, String> {
    self
      .canonical_digits
      .get(&value)
      .ok_or(String::from("Canonical digit missing"))
  }

  fn known_mappings(&self) -> Vec<(char, char)> {
    self
      .segment_candidate_mappings
      .iter()
      .filter(|(_, c)| c.len() == 1)
      .map(|(&s, c)| (s, c.iter().nth(0).unwrap().clone()))
      .collect()
  }

  fn update_mappings_for_digit(&mut self, digit: &DigitalOutput) -> Result<(), String> {
    // Filter by segment count
    let mut candidate_values: HashSet<usize> =
      if !self.disabled_features.contains(ALG_FEATURE_SEGMENT_COUNT) {
        self
          .canonical_digits
          .iter()
          .filter(|(_, d)| d.segments.len() == digit.segments.len())
          .map(|(&v, _)| v)
          .collect()
      } else {
        initialise_values()
      };

    // Filter by known mappings
    if !self
      .disabled_features
      .contains(ALG_FEATURE_OVERLAP_MAPPINGS)
    {
      for (scrambled, canonical) in self.known_mappings() {
        let mut updated_candidates = candidate_values.clone();
        for value in candidate_values {
          let canonical_digit = self.get_canonical_digit(value)?;
          if digit.segments.contains(&scrambled) != canonical_digit.segments.contains(&canonical) {
            updated_candidates.remove(&value);
          }
        }
        candidate_values = updated_candidates;
      }
    }

    // Update mappings
    let mut candidate_canonical_segments: HashSet<char> = HashSet::new();
    for value in candidate_values {
      for &c in &self.get_canonical_digit(value)?.segments {
        candidate_canonical_segments.insert(c);
      }
    }
    for &scrambled in &digit.segments {
      let canonical = self
        .segment_candidate_mappings
        .remove(&scrambled)
        .ok_or(String::from("Segment missing from mappings"))?;
      let mut updated_canonical = canonical.clone();
      for c in canonical {
        if !candidate_canonical_segments.contains(&c) {
          updated_canonical.remove(&c);
        }
      }
      self
        .segment_candidate_mappings
        .insert(scrambled, updated_canonical);
    }

    Ok(())
  }

  fn count_candidates_which_subset(&self, superset: &HashSet<char>) -> usize {
    self
      .segment_candidate_mappings
      .values()
      .filter(|s| s.is_subset(superset))
      .count()
  }

  fn count_candidates_which_overlap(&self, set: &HashSet<char>) -> usize {
    self
      .segment_candidate_mappings
      .values()
      .filter(|s| !s.is_disjoint(set))
      .count()
  }

  fn reduce_exclusive_groupings(&mut self) -> bool {
    let mut do_update: Option<HashSet<char>> = None;
    for candidates in self.segment_candidate_mappings.values() {
      if self.count_candidates_which_subset(candidates) == candidates.len()
        && self.count_candidates_which_overlap(candidates) > candidates.len()
      {
        do_update = Some(candidates.clone());
        break;
      }
    }

    match do_update {
      Some(u) => {
        for candidates in self.segment_candidate_mappings.values_mut() {
          if !candidates.is_disjoint(&u) && !candidates.is_subset(&u) {
            for c in &u {
              candidates.remove(c);
            }
          }
        }

        true
      }
      None => false,
    }
  }

  fn produce_solution(self, final_output: [DigitalOutput; 4]) -> Result<Solution, String> {
    let canonical_to_scrambled: HashMap<char, char> = self
      .known_mappings()
      .iter()
      .cloned()
      .map(|(s, c)| (c, s))
      .collect();
    let mut scrambled_digit_mappings: HashMap<String, usize> = HashMap::new();
    for (&value, canonical) in self.canonical_digits {
      let mut segments: HashSet<char> = HashSet::new();
      for c in &canonical.segments {
        let scrambled = canonical_to_scrambled
          .get(&c)
          .ok_or(String::from("Mapping not known for segment"))?
          .clone();
        segments.insert(scrambled);
      }
      scrambled_digit_mappings.insert(DigitalOutput::new(segments).canonical, value);
    }

    Ok(Solution::new(final_output, scrambled_digit_mappings))
  }
}

pub struct Solver {
  canonical_digits: HashMap<usize, DigitalOutput>,
  disabled_features: HashSet<String>,
}

impl Solver {
  pub fn new(
    canonical_digits: HashMap<usize, DigitalOutput>,
    disabled_features: HashSet<String>,
  ) -> Solver {
    Solver {
      canonical_digits,
      disabled_features,
    }
  }

  pub fn analyze_displays(&self, display: DisplayAnalysis) -> Result<Solution, String> {
    let mut internal = SolverInternal::new(&self.canonical_digits, &self.disabled_features);
    let sample_outputs = display.sample_outputs;
    if !self
      .disabled_features
      .contains(ALG_FEATURE_FREQUENCY_ANALYSIS)
    {
      internal.reduce_segments_by_frequency(sample_outputs.iter().collect())?;
    }
    let mut loop_count: i32 = 0;
    while internal.known_mappings().len() < internal.segment_candidate_mappings.len() {
      if loop_count > MAX_LOOP {
        panic!("Too many iterations in solver loop");
      }
      loop_count += 1;
      for digit in &sample_outputs {
        internal.update_mappings_for_digit(digit)?;
      }
      if !self
        .disabled_features
        .contains(ALG_FEATURE_SUBSET_CANDIDATES)
      {
        while internal.reduce_exclusive_groupings() {}
      }
    }

    internal.produce_solution(display.final_output)
  }
}
