use std::cmp::Ordering;

//#[derive(Eq)]
pub struct PQEntry{
    pub margin:f32,
    pub node_offset:i64,
}

impl PQEntry{
    pub fn new(margin:f32, node_offset:i64) -> PQEntry{
        return PQEntry{
            margin: margin,
            node_offset: node_offset,
        };
    }
}

/*
impl Ord for PQEntry{
    fn cmp(&self, other: &PQEntry) -> Ordering {
        &other.margin.cmp(&self.margin);
    }
}

impl PartialOrd for PQEntry {
    fn partial_cmp(&self, other: &PQEntry) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for PQEntry {
    fn eq(&self, other: &PQEntry) -> bool {
        self.margin == other.margin
    }
}
*/