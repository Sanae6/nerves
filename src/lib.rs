pub trait Nerve {
  type Host;
  fn execute(&self, host: &mut Self::Host, ctx: &mut NerveContext<Self::Host>);
}

#[macro_export]
macro_rules! new_nerves {
  ($host: ty, $($action: ident),+) => {
    $(
      ::paste::paste! {
        struct [<Nrv $action:camel>];

        impl ::nerves_for_jenn::Nerve for [<Nrv $action:camel>] {
          type Host = $host;

          fn execute(
            &self,
            host: &mut Self::Host,
            ctx: &mut ::nerves_for_jenn::NerveContext<Self::Host>
          ) {
            host.[<exe_ $action:snake>](ctx);
          }
        }
      }
    )+
  };
}

pub struct NerveContext<H: 'static> {
  current_nerve: &'static dyn Nerve<Host = H>,
  next_nerve: Option<&'static dyn Nerve<Host = H>>,
  step: usize,
}

impl<H: 'static> NerveContext<H> {
  pub fn step(&self) -> usize {
    self.step
  }

  pub fn first_step(&self) -> bool {
    self.step == 0
  }

  pub fn set_nerve(&mut self, new_nerve: &'static dyn Nerve<Host = H>) {
    self.next_nerve = Some(new_nerve);
  }

  pub fn current_nerve(&self) -> &'static dyn Nerve<Host = H> {
    self.current_nerve
  }

  pub fn restart_nerve(&mut self) {
    self.set_nerve(self.current_nerve);
  }
}

pub struct NerveExecutor<H: 'static> {
  host: H,
  nerve: &'static dyn Nerve<Host = H>,
  step: usize,
}

impl<H> NerveExecutor<H> {
  pub fn new(host: H, nerve: &'static dyn Nerve<Host = H>) -> Self {
    Self {
      host,
      nerve,
      step: 0,
    }
  }

  pub fn host(&self) -> &H {
    &self.host
  }

  pub fn host_mut(&mut self) -> &mut H {
    &mut self.host
  }

  pub fn update(&mut self) {
    let mut context = NerveContext {
      current_nerve: self.nerve,
      next_nerve: None,
      step: self.step,
    };
    self.nerve.execute(&mut self.host, &mut context);
    self.step += 1;
    if let Some(next_nerve) = context.next_nerve {
      self.nerve = next_nerve;
      self.step = 0;
    }
  }
}
