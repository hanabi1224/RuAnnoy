pub struct PriorityQueueEntry {
    pub margin: f32,
    pub node_offset: i64,
}

impl PriorityQueueEntry {
    pub fn new(margin: f32, node_offset: i64) -> PriorityQueueEntry {
        return PriorityQueueEntry {
            margin: margin,
            node_offset: node_offset,
        };
    }
}
