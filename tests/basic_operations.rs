use circuits::{Gate, AndGate, OrGate, NotGate, NandGate, NorGate, XorGate};

struct Input{ x: i8, y: i8, output: i8 }

#[test]
fn test_and_gate() {
  let inputs: Vec<Input> = vec!(
    Input{ x: 1, y: 0, output: 0 },
    Input{ x: 0, y: 1, output: 0 },
    Input{ x: 1, y: 1, output: 1 },
    Input{ x: 0, y: 0, output: 0 },
    Input{ x: 0, y: 1, output: 0 },
  );
  for input in inputs {
      let output = AndGate { x: input.x, y: input.y }.process();
      assert_eq!(output, input.output);
  }
}

#[test]
fn test_or_gate() {
  let inputs: Vec<Input> = vec!(
    Input{ x: 1, y: 0, output: 1 },
    Input{ x: 0, y: 1, output: 1 },
    Input{ x: 1, y: 1, output: 1 },
    Input{ x: 0, y: 0, output: 0 },
    Input{ x: 0, y: 1, output: 1 },
  );
  for input in inputs {
      let output = OrGate { x: input.x, y: input.y }.process();
      assert_eq!(output, input.output);
  }
}

#[test]
fn test_not_gate() {
  let inputs: Vec<Input> = vec!(
    Input{ x: 1, y: 1, output: 0 },
    Input{ x: 0, y: 0, output: 1 },
  );
  for input in inputs {
      let output = NotGate { x: input.x }.process();
      assert_eq!(output, input.output);
  }
}

#[test]
fn test_nand_gate() {
  let inputs: Vec<Input> = vec!(
    Input{ x: 1, y: 0, output: 1 },
    Input{ x: 0, y: 1, output: 1 },
    Input{ x: 1, y: 1, output: 0 },
    Input{ x: 0, y: 0, output: 1 },
    Input{ x: 0, y: 1, output: 1 },
  );
  for input in inputs {
      let output = NandGate { x: input.x, y: input.y }.process();
      assert_eq!(output, input.output);
  }
}

#[test]
fn test_nor_gate() {
  let inputs: Vec<Input> = vec!(
    Input{ x: 1, y: 0, output: 0 },
    Input{ x: 0, y: 1, output: 0 },
    Input{ x: 1, y: 1, output: 0 },
    Input{ x: 0, y: 0, output: 1 },
    Input{ x: 0, y: 1, output: 0 },
  );
  for input in inputs {
      let output = NorGate { x: input.x, y: input.y }.process();
      assert_eq!(output, input.output);
  }
}

#[test]
fn test_xor_gate() {
  let inputs: Vec<Input> = vec!(
    Input{ x: 1, y: 0, output: 1 },
    Input{ x: 0, y: 1, output: 1 },
    Input{ x: 1, y: 1, output: 0 },
    Input{ x: 0, y: 0, output: 0 },
    Input{ x: 0, y: 1, output: 1 },
  );
  for input in inputs {
      let output = XorGate { x: input.x, y: input.y }.process();
      assert_eq!(output, input.output);
  }
}
