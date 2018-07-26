#[cfg(test)]
mod tests {
    extern crate ru_annoy;
    use self::ru_annoy::{IndexType,AnnoyIndex,AnnoyIndexSearchApi};

    use std::vec::Vec;

    #[test]
    fn sanity_tests() {
        let index = AnnoyIndex::new(10,  "tests/test.10d.ann", IndexType::Angular);
        assert_eq!(index.get_item_vector(0), [-0.49093127250671387, 0.11732950061559677, -0.9871269464492798, -0.7244759798049927, 0.38621339201927185, 0.17796599864959717, 1.3940260410308838, -0.12950724363327026, 0.2716858386993408, -0.5863288640975952]);
        assert_eq!(index.get_item_vector(4), [-0.3540892004966736, -0.6328534483909607, 0.08625798672437668, 0.7626655101776123, 0.6639019846916199, -1.295175313949585, 1.5552952289581299, 1.4021003246307373, 0.41959965229034424, -0.7930657863616943]);

        let v1 = index.get_item_vector(1);
        let nearest = index.get_nearest(v1.as_ref(), 5, -1, true);
        let mut id_list:Vec<i64> = Vec::new();
        let mut distance_list:Vec<f32> = Vec::new();
        for item in nearest{
            id_list.push(item.id);
            distance_list.push(item.distance);
        }

        assert_eq!(id_list, [1, 19, 3, 62, 77]);
        assert_eq!(distance_list, [0.0, 0.7615587, 0.8742371, 1.0227013, 1.04167736]);

        assert_eq!(2 + 3, 5);
    }
}
