#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32
}

#[derive(Debug)]
pub struct AssignmentPair {
    a: Assignment,
    b: Assignment
}

impl Assignment {
    pub fn from(assignment: &str) -> Assignment {
        let assignments: Vec<&str> = assignment.split("-").collect();
        let start = u32::from_str_radix(&assignments[0], 10).unwrap();
        let end = u32::from_str_radix(&assignments[1], 10).unwrap();

        Assignment { start, end }
    }

    fn envelops(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        (self.start <= other.start && self.end >= other.start)
            || (self.end >= other.end && self.start <= other.end)
            || (self.start >= other.start && self.end <= other.end)
    }
}

impl AssignmentPair {
    pub fn from(line: &str) -> AssignmentPair {
        let pair: Vec<&str> = line.split(",").collect();

        assert_eq!(pair.len(), 2);

        AssignmentPair { 
            a: Assignment::from(&pair[0]),
            b: Assignment::from(&pair[1])
        }
    }

    pub fn one_assignment_envelops_another(&self) -> bool {
        self.a.envelops(&self.b) || self.b.envelops(&self.a)
    }

    pub fn has_overlapping_assignments(&self) -> bool {
        self.a.overlaps(&self.b)
    }
}

#[test]
fn test_from() {
    let pair = AssignmentPair::from("2-4,6-8");

    assert_eq!(pair.a.start, 2);
    assert_eq!(pair.a.end, 4);
    assert_eq!(pair.b.start, 6);
    assert_eq!(pair.b.end, 8);
}

#[test]
fn test_overlaps() {
    let non_overlapping_pair = AssignmentPair::from("2-4,6-8");
    let overlapping_pair_1 = AssignmentPair::from("2-4,4-8");
    let overlapping_pair_2 = AssignmentPair::from("2-4,1-2");
    let overlapping_pair_3 = AssignmentPair::from("4-8,5-7");
    let overlapping_pair_4 = AssignmentPair::from("2-3,1-4");
    let overlapping_pair_5 = AssignmentPair::from("1-4,2-3");

    assert_eq!(non_overlapping_pair.has_overlapping_assignments(), false);
    assert_eq!(overlapping_pair_1.has_overlapping_assignments(), true);
    assert_eq!(overlapping_pair_2.has_overlapping_assignments(), true);
    assert_eq!(overlapping_pair_3.has_overlapping_assignments(), true);
    assert_eq!(overlapping_pair_4.has_overlapping_assignments(), true);
    assert_eq!(overlapping_pair_5.has_overlapping_assignments(), true);
}