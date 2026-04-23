use std::{
  any::{Any, TypeId},
  cell::RefCell,
};

pub trait Nerve {
  type Host;
  fn execute(&self, host: &mut Self::Host);
}

#[macro_export]
macro_rules! new_nerves {
  ($host: ty, $($action: ident),+) => {
    $(
      ::paste::paste! {
        struct [<Nrv $action:camel>];

        impl ::nerves_for_jenn::Nerve for [<Nrv $action:camel>] {
          type Host = $host;

          fn execute(&self, host: &mut Self::Host) {
            host.[<exe_ $action:snake>]();
          }
        }
      }
    )+
  };
}

struct NextNerve {
  ty: TypeId,
  nerve: [u8; size_of::<*const dyn Nerve<Host = ()>>()],
}

impl NextNerve {
  pub fn new<H: 'static>(nerve: &'static impl Nerve<Host = H>) -> Self {
    let ty = TypeId::of::<H>();

    let nerve: &'static dyn Nerve<Host = H> = nerve;
    let nerve = unsafe { std::mem::transmute(nerve as *const dyn Nerve<Host = H>) };

    Self { ty, nerve }
  }

  pub fn try_into<H: 'static>(self) -> &'static dyn Nerve<Host = H> {
    assert_eq!(self.ty, TypeId::of::<H>());

    let nerve: &'static dyn Nerve<Host = H> = unsafe { std::mem::transmute(self.nerve) };
    nerve
  }
}

#[derive(Default)]
struct NerveCtx {
  next_nerve: Option<NextNerve>,
  step: u32,
}

thread_local! {
  static CONTEXT: RefCell<Vec<NerveCtx>> = RefCell::new(Vec::new());
}

impl NerveCtx {
  fn push_context(self) {
    CONTEXT.with(|context| context.borrow_mut().push(self));
  }

  fn pop_context() -> Self {
    CONTEXT.with(|context| context.borrow_mut().pop().expect("not in a nerve??"))
  }

  fn with<R>(func: impl FnOnce(&mut NerveCtx) -> R) -> R {
    CONTEXT.with(|context| func(context.borrow_mut().last_mut().expect("not in a nerve")))
  }

  pub fn set_nerve<H: 'static>(new_nerve: &'static impl Nerve<Host = H>) {
    let nerve = NextNerve::new(new_nerve);
    Self::with(|ctx| ctx.next_nerve = Some(nerve));
  }

  pub fn get_step() -> u32 {
    Self::with(|ctx| ctx.step)
  }
}

pub mod al {
  use crate::{Nerve, NerveCtx};

  pub fn set_nerve<H: 'static>(_host: &H, new_nerve: &'static impl Nerve<Host = H>) {
    NerveCtx::set_nerve(new_nerve);
  }

  pub fn get_step(_host: &impl Sized) -> u32 {
    NerveCtx::get_step()
  }
}

pub struct NerveExecutor<H: 'static> {
  host: H,
  nerve: &'static dyn Nerve<Host = H>,
  step: u32,
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
    NerveCtx {
      next_nerve: None,
      step: self.step,
    }
    .push_context();
    self.nerve.execute(&mut self.host);
    self.step += 1;
    let ctx = NerveCtx::pop_context();
    if let Some(next_nerve) = ctx.next_nerve {
      self.nerve = next_nerve.try_into::<H>();
      self.step = 0;
    }
  }
}

