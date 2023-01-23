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
}

#[test]
fn test_from() {
    let pair = AssignmentPair::from("2-4,6-8");

    assert_eq!(pair.a.start, 2);
    assert_eq!(pair.a.end, 4);
    assert_eq!(pair.b.start, 6);
    assert_eq!(pair.b.end, 8);
}

