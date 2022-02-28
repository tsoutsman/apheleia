use crate::graphql::SubjectArea;

pub(crate) fn schemas(subject_areas: Option<Vec<SubjectArea>>) -> Vec<SubjectArea> {
    match subject_areas {
        // Could be exploited with very large vec
        Some(v) => {
            let capacity = std::cmp::min(v.len(), SubjectArea::SIZE);

            let mut result = Vec::with_capacity(capacity);
            let mut set = hashbrown::HashSet::with_capacity(capacity);

            for area in v {
                if set.insert(area) {
                    result.push(area);
                }
            }

            result
        }
        None => SubjectArea::all().to_vec(),
    }
}
