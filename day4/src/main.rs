use std::fs;

struct SectionAssignment {
    section1: (i32, i32),
    section2: (i32, i32),
}

fn main() {
    let input = fs::read_to_string("src/input.txt")
        .expect("File reading failed");
    let input: Vec<&str> = input.split('\n').collect();

    let mut assignments: Vec<SectionAssignment> = vec![];
    for assignment in &input {
        let a: Vec<i32> = assignment.split(|x| x ==',' || x == '-').map(|x| x.parse::<i32>().unwrap()).collect();
        assignments.push(SectionAssignment { section1: (a[0], a[1]), section2: (a[2], a[3]) })
    }

    let mut p1_counter = 0;
    let mut p2_counter = 0;
    for assignment in &assignments {
        if (assignment.section1.0 >= assignment.section2.0 &&
            assignment.section1.1 <= assignment.section2.1) || 
            (assignment.section1.0 <= assignment.section2.0 &&
            assignment.section1.1 >= assignment.section2.1) {
                p1_counter += 1;
                println!("p1 - ({},{})({},{})", assignment.section1.0, assignment.section1.1, assignment.section2.0, assignment.section2.1);
            }

        if assignment.section1.1 < assignment.section2.0 ||
            assignment.section2.1 < assignment.section1.0 {
                // p2_counter += 1;
                println!("p2 - ({},{})({},{})", assignment.section1.0, assignment.section1.1, assignment.section2.0, assignment.section2.1);
            }
        else {p2_counter += 1};
    }

    println!("{}", p1_counter);
    println!("{}", p2_counter);

    


}
