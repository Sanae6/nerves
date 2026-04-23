use std::{future::ready, task::Poll};

use nerves_for_jenn::{NerveExecutor, al, new_nerves};

struct Test {
  stop: bool,
}

impl Test {
  pub fn exe_start(&mut self) {
    println!("start");
    al::set_nerve(self, &NrvCounting);
  }

  pub fn exe_counting(&mut self) {
    if al::get_step(self) >= 5 {
      al::set_nerve(self, &NrvStop);
    } else {
      println!("counting (step: {})", al::get_step(self) + 1)
    }
  }

  pub fn exe_stop(&mut self) {
    println!("done");
    self.stop = true;
  }
}

new_nerves!(Test, Start, Counting, Stop);

fn main() {
  let mut exec = NerveExecutor::new(Test { stop: false }, &NrvStart);

  while !exec.host().stop {
    exec.update();
  }
}
