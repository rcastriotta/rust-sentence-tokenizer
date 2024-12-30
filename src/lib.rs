#![cfg_attr(test, feature(test))]
#![warn(missing_docs)]

extern crate freqdist;
extern crate num;
extern crate phf;
extern crate rustc_serialize;
#[cfg(test)]
extern crate test;
#[cfg(test)]
extern crate walkdir;

mod prelude;
mod token;
mod tokenizer;
mod trainer;
mod util;

pub use tokenizer::{SentenceByteOffsetTokenizer, SentenceTokenizer};
pub use trainer::{Trainer, TrainingData};

/// Contains traits for configuring all tokenizers, and the trainer. Also
/// contains default parameters for tokenizers, and the trainer.
pub mod params {
  pub use prelude::{
    DefinesInternalPunctuation, DefinesNonPrefixCharacters, DefinesNonWordCharacters,
    DefinesPunctuation, DefinesSentenceEndings, Set, Standard, TrainerParameters,
  };
}

#[cfg(test)]
fn get_test_scenarios(dir_path: &str, raw_path: &str) -> Vec<(Vec<String>, String, String)> {
  #![allow(unused_must_use)]

  use std::fs;
  use std::io::Read;
  use std::path::Path;

  use walkdir::WalkDir;

  let mut tests = Vec::new();

  for path in WalkDir::new(dir_path) {
    let entry = path.unwrap();
    let fpath = entry.path();

    if fpath.is_file() {
      let mut exp_strb = String::new();
      let mut raw_strb = String::new();

      // Files in the directory with raw articles must match the file names of
      // articles in the directory with test outcomes.
      let rawp = Path::new(raw_path).join(fpath.file_name().unwrap());

      fs::File::open(&fpath)
        .unwrap()
        .read_to_string(&mut exp_strb);
      fs::File::open(&rawp).unwrap().read_to_string(&mut raw_strb);

      // Expected results, split by newlines.
      let exps: Vec<String> = exp_strb.split('\n').map(|s| s.to_string()).collect();

      tests.push((exps, raw_strb, format!("{:?}", fpath.file_name().unwrap())));
    }
  }

  tests // Returns (Expected cases, File contents, File name)
}
