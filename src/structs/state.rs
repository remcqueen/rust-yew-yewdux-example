use yewdux::store::Store;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct CounterState {
    pub count: u32,
}
