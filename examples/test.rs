use nerves_for_jenn::{NerveContext, NerveExecutor, new_nerves};

struct Test {
  stop: bool,
}

impl Test {
  pub fn exe_start(&mut self, context: &mut NerveContext<Self>) {
    println!("start");
    context.set_nerve(&NrvCounting);
  }

  pub fn exe_counting(&mut self, context: &mut NerveContext<Self>) {
    if context.step() >= 5 {
      context.set_nerve( &NrvStop);
    } else {
      println!("counting (step: {})", context.step() + 1)
    }
  }

  pub fn exe_stop(&mut self, _context: &mut NerveContext<Self>) {
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
