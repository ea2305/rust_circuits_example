pub trait Gate {
  fn process(&self) -> i8;
}

pub struct AndGate {
  pub x: i8,
  pub y: i8
}

impl Gate for AndGate {
  fn process(&self) -> i8 {
    self.x * self.y
  }
}

pub struct OrGate {
  pub x: i8,
  pub y: i8
}

impl Gate for OrGate {
  fn process(&self) -> i8 {
    let acc = self.x + self.y;
    if acc > 0 {
        return 1;
    } else {
        return 0;
    }
  }
}

pub struct NotGate {
  pub x: i8
}

impl Gate for NotGate {
  fn process(&self) -> i8 {
    if self.x == 1 {
      return 0;
    } else {
      return 1;
    }
  }
}

pub struct NandGate {
  pub x: i8,
  pub y: i8
}

impl Gate for NandGate {
  fn process(&self) -> i8 {
    let preprocess = AndGate {x: self.x, y: self.y };
    let pulse = preprocess.process();

    let refinament = NotGate { x: pulse };
    return refinament.process();
  }
}

pub struct NorGate {
  pub x: i8,
  pub y: i8
}

impl Gate for NorGate {
  fn process(&self) -> i8 {
    let preprocess = OrGate {x: self.x, y: self.y };
    let pulse = preprocess.process();

    let refinament = NotGate { x: pulse };
    return refinament.process();
  }
}

pub struct XorGate {
  pub x: i8,
  pub y: i8
}

impl Gate for XorGate {
  fn process(&self) -> i8 {
    let ax = NotGate{ x: self.x }.process();
    let ay = self.y;

    let bx = self.x;
    let by = NotGate{ x: self.y }.process();

    let a_input = AndGate{ x: ax, y: ay }.process();
    let b_input = AndGate{ x: bx, y: by }.process();

    return OrGate{ x: a_input, y: b_input }.process();
  }
}
