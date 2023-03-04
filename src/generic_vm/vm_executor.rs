use crate::generic_vm::vm_state::VMStateT;
use crate::input::VMInputT;
use crate::state_input::StagedVMState;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

pub const MAP_SIZE: usize = 1024;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionResult<VS>
where
    VS: Default + VMStateT,
{
    pub output: Vec<u8>,
    pub reverted: bool,
    #[serde(deserialize_with = "StagedVMState::deserialize")]
    pub new_state: StagedVMState<VS>,
}

impl<VS> ExecutionResult<VS>
where
    VS: Default + VMStateT + 'static,
{
    pub fn empty_result() -> Self {
        Self {
            output: vec![],
            reverted: false,
            new_state: StagedVMState::new_uninitialized(),
        }
    }
}

pub trait GenericVM<VS, Code, By, Loc, SlotTy, I, S> {
    fn deploy(&mut self, code: Code, constructor_args: Option<By>, deployed_address: Loc) -> Option<Loc>;
    fn execute(&mut self, input: &I, state: Option<&mut S>) -> ExecutionResult<VS>
    where
        VS: VMStateT;

    // all these method should be implemented via a global variable, instead of getting data from
    // the `self`. `self` here is only to make the trait object work.
    fn get_jmp(&self) -> &'static mut [u8; MAP_SIZE];
    fn get_read(&self) -> &'static mut [bool; MAP_SIZE];
    fn get_write(&self) -> &'static mut [u8; MAP_SIZE];
    fn get_cmp(&self) -> &'static mut [SlotTy; MAP_SIZE];
    fn state_changed(&self) -> bool;
}
