// Possible algorithm:
// 1. For each pair of points in a scanner, calclate the distance between them. If the distance
// is <= 1000, store it.
// 2. Define scanner 0 as "solved" with 0 rotations or translations. Define all other scanners as
// "unsolved".
// 3. While there are still unsolved scanners, pick 1 unsolved scanner & 1 solved scanner. Attempt
// to determine if they have at least 12 matching points according to the matching sub-algorithm.
// 4. If they do not match, cache this result. Select a different solved scanner & attempt to match
// it to the unsolved scanner.
// 5. If there are no more solved scanners, pick a different unsolved scanner & attempt to match it
// to any of the solved scanners.
// 6. When an unsolved scanner is matched to a solved scanner, create a new "solved" record reflecting
// the points in the unsolved scanner after applying the rotations/translations required to match it.
// Remove the unsolved scanner.

// Matching sub-algorithm:
// 1. Consider the set of point-pair distances. If there are not at least 78 common lengths between
// the two scanners, they do not match.
// 2. Select a length & inspect the points associated with it (1). Attempt to create a mapping between
// the solved points & unsolved points - defined by rotations + translations. There should be 2 * N
// possible mappings, where N is the number of times that length appears.
// 3. Repeat this process for the other lengths; each should produce at least 1 transformation which
// has already been computed by another length consideration. Filter down to a transformation which is
// consistent with all possible point mappings. If there are none, the scanners do not match. (2)

// (1) Sort these lengths by number of occurances. This should ideally result in N=1 for each length
// that needs to be considered, but this is not guaranteed.
// To keep things simple, we'll assume this is the case unless proven otherwise.
//
// (2) Similar to (1), we will probably stop considering lengths once we have reached 1 possible
// transformation. The remaining lengths will just be confirmed against this transformation; this
// should keep the computation time down.

use ndarray::{Array1, Array2};

use crate::day19::scanner::Scanner;

struct Transform {
  translation: Array1<i32>,
  rotation: Array2<i32>,
}

fn derive_match(solved: &Scanner, unsolved: &Scanner) -> Option<Transform> {
  // let mut lengths: Vec<i32> = unsolved.
  None
}

struct Solver {
  solved: Vec<Scanner>,
  unsolved: Vec<Scanner>,
}

impl Solver {
  fn new(mut scanners: Vec<Scanner>) -> Solver {
    let solved = match scanners.pop() {
      Some(s) => vec![s],
      None => vec![],
    };
    Solver {
      solved,
      unsolved: scanners,
    }
  }
}

pub fn solve_and_count_points(scanners: Vec<Scanner>) -> Result<i32, String> {
  Err(format!("TODO implement"))
}
