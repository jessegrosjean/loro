use fxhash::FxHashMap;
use rle::RleVec;

use smallvec::SmallVec;

use crate::{
    container::{Container, ContainerID, ContainerType},
    id::ID,
    log_store::LogStoreWeakRef,
    op::OpProxy,
    span::IdSpan,
    value::LoroValue,
    ClientID,
};

#[derive(Clone, Debug)]
struct DagNode {
    id: IdSpan,
    deps: SmallVec<[ID; 2]>,
}

#[derive(Clone, Debug)]
pub struct TextContainer {
    id: ContainerID,
    sub_dag: FxHashMap<ClientID, RleVec<DagNode, ()>>,
    log_store: LogStoreWeakRef,
}

impl TextContainer {
    pub fn insert(&mut self, _pos: usize, _text: &str) -> ID {
        todo!()
    }

    pub fn delete(&mut self, _pos: usize, _len: usize) {}
}

impl Container for TextContainer {
    fn id(&self) -> &ContainerID {
        &self.id
    }

    fn type_(&self) -> ContainerType {
        ContainerType::Text
    }

    fn apply(&mut self, _op: &OpProxy) {
        todo!()
    }

    fn checkout_version(&mut self, _vv: &crate::VersionVector) {
        todo!()
    }

    fn get_value(&mut self) -> &LoroValue {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
