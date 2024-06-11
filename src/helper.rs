pub struct NeighboursAndWeights {
    pub p: Vec<usize>, // Neighbour
    pub w: Vec<f64>,   // Weight associated with Neighbour
}

impl NeighboursAndWeights {
    // Construct <(patch id, weight)> on self excluding weights and patch ids of patches in neighbours_to_ignore vec
    // If called on a PandW that contains no elements in p or w it will return an empty list.
    // The returned f64 weight associated with the patch id is calcated as w / all other weights (ignoring any weights and patch ids belonginging to patches in neighbours_ignore)
    pub fn construct_tuple_arr(
        &self,
        neighbours_to_exclude: Option<&Vec<usize>>,
    ) -> Vec<(usize, f64)> {
        // Here we get patch index's and weight in a tuple array for all patches belonging to self, not present in neighbours_to_ignore
        let p_index_and_weight: Vec<(usize, &f64)> = self
            .w
            .iter()
            .enumerate()
            // filter out any patches that are in neighbours_to_ignore, if none passed in filter on !false, if there are elements passed subtract 1 from data in PandW to match boar patch ids
            .filter(|(index, _)| {
                !neighbours_to_exclude
                    .map_or(false, |nbr_arr| nbr_arr.contains(&(self.p[*index] - 1)))
            }) //JS -1 is due to negating one from other data when it is loaded
            .collect();

        let mut ans: Vec<(usize, f64)> = Vec::with_capacity(p_index_and_weight.len());
        // here we loop through all created patch and weight tuples and insert the patch at the given index along with its weight / all other weights into ans array
        for (patch_index, weight) in p_index_and_weight.iter() {
            ans.push((
                self.p[*patch_index], // Get patch id using index
                **weight / (p_index_and_weight.iter().fold(0f64, |sum, (_, w)| sum + *w)),
            ));
        }
        ans
    }
}
