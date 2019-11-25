extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Rem;

impl<E, O> VMInstruction<E, O> for Rem
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;
        let (_div, rem) = vm.get_operator().div_rem(left, right)?;

        vm.stack_push(rem)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_rem() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 4.into() })
            .add(Push { value: 9.into() })
            .add(Rem)
            .add(Push { value: (-4).into() })
            .add(Push { value: 9.into() })
            .add(Rem)
            .add(Push { value: 4.into() })
            .add(Push { value: (-9).into() })
            .add(Rem)
            .add(Push { value: (-4).into() })
            .add(Push { value: (-9).into() })
            .add(Rem)
            .test(&[3, 3, 1, 1])
    }
}
