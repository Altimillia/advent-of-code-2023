use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::ops::Range;
use nom::bytes::complete::{tag, take, take_till, take_until};
use nom::IResult;
use crate::tools::parse_numbers;

pub fn part_one(input: String) -> impl Display {
    factory_line(input)
}

fn factory_line(input: String) -> i32 {
    let mut accepted:Vec<Part> = vec![];
    let mut rejected:Vec<Part> = vec![];
    let (workflows, parts) = parse_information(input);

    for part in parts {
        let mut active_workflow = workflows.get("in").unwrap();

        loop {
            let destination = active_workflow.get_part_destination(&part);

            match destination.as_str() {
                "A" => {
                    accepted.push(part);
                    break;
                }
                "R" => {
                    rejected.push(part);
                    break;
                }
                _ => {
                    active_workflow = workflows.get(destination.as_str()).unwrap();
                }
            }
        }
    }


    accepted.iter().map(|part| part.sum()).sum::<i32>()
}

fn factory_line_2(input: String) -> u128 {
    let mut accepted:Vec<TheoreticalPart> = vec![];
    let mut rejected:Vec<TheoreticalPart> = vec![];
    let mut part_queue:VecDeque<(TheoreticalPart, String)> = VecDeque::new();

    part_queue.push_back((TheoreticalPart::blank(), "in".to_string()));

    let (workflows, _) = parse_information(input);

    while part_queue.len() > 0 {
        let (next_part, dest) = part_queue.pop_front().unwrap();

        if dest == "A" {
            accepted.push(next_part);
            continue;
        }
        if dest == "R" {
            rejected.push(next_part);
            continue;
        }

        let workflow = workflows.get(dest.as_str()).unwrap();
        let results = workflow.process_theoretical_part(&next_part);

        println!("{}", results.len());
        for result in results {
            part_queue.push_back(result);
        }

    }

    println!("{}", accepted.len());
    let mut max_value:u128 = 0;

    for part in &accepted {
        part.print();
        max_value += part.get_combos();
        //println!("{}", part.get_combos());
    }

    max_value
}

fn parse_information(input: String) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut split = input.split_terminator("\n\n");
    let workflow_lines = split.nth(0).unwrap().lines();
    let part_lines = split.nth(0).unwrap().lines();

    let mut workflow_map:HashMap<String, Workflow> = HashMap::new();
    workflow_lines.for_each(|line| {
        let workflow = Workflow::parse(line).unwrap().1;

        workflow_map.insert(workflow.name.clone(), workflow);
    });
    let parts = part_lines.map(|line| Part::parse(line)).collect();

    return (workflow_map,parts);
}

pub fn part_two(input: String) -> impl Display {
    factory_line_2(input)
}

struct Workflow {
    name: String,
    rules: Vec<WorkflowRule>
}

impl Workflow {
    fn parse(input_line: &str) -> IResult<&str, Self> {
        let (input_line, name) = take_until("{")(input_line)?;
        let (input_line, _) = tag("{")(input_line)?;
        let (input_line, rules_string) = take_until("}")(input_line)?;

        let rules = rules_string.split(",").map(|rule| WorkflowRule::parse(rule).unwrap().1).collect();

        Ok((input_line, Workflow { name: name.to_string(), rules }))
    }

    fn get_part_destination(&self, part: &Part) -> String {
        for rule in &self.rules {
            if rule.can_apply(&part) {
                return rule.destination.clone();
            }
        }
        panic!("Shouldn't happen")
    }

    fn process_theoretical_part(&self, part: &TheoreticalPart) -> Vec<(TheoreticalPart, String)> {
        let mut results:Vec<(TheoreticalPart, String)> = Vec::new();

            let mut next_part = part.clone();
            for rule in &self.rules {
                if rule.can_apply_in_theory(&next_part) {
                    if rule.requirement.is_some() {
                        let updated =  next_part.split_part(rule.requirement.unwrap());
                        println!("{} {} {}", rule.requirement.unwrap().part_id, rule.requirement.unwrap().operator, rule.requirement.unwrap().part_req);
                        updated[0].print();
                        results.push((updated[0].clone(), rule.destination.to_string()));
                        if updated.len() > 1 {
                            updated[1].print();
                            next_part = updated[1].clone();
                        }
                    }
                    else {
                        results.push((next_part.clone(), rule.destination.to_string()));
                    }
                }
            }

        results
    }
}

#[derive(Debug, PartialEq, Clone)]
struct TheoreticalPart {
    values: HashMap<char, Range<i32>>
}
impl TheoreticalPart {
    fn blank() -> Self {
        let mut values:HashMap<char, Range<i32>> = HashMap::new();

        values.insert('x', Range { start: 1, end: 4000});
        values.insert('m', Range { start: 1, end: 4000});
        values.insert('a', Range { start: 1, end: 4000});
        values.insert('s', Range { start: 1, end: 4000});

        TheoreticalPart { values }
    }

    fn split_part(&self, operation: Operation) -> Vec<TheoreticalPart> {
        if let Some(val) = self.values.get(&operation.part_id) {

            if !val.contains(&operation.part_req) {
                return vec![self.clone()]
            }

            let (r1,r2) = match operation.operator {
                '<' => (Range { start: val.start, end: operation.part_req - 1 }, Range { start: operation.part_req, end: val.end }),
                '>' => (Range { start: operation.part_req + 1, end: val.end }, Range { start: val.start, end: operation.part_req }),
                _ => panic!("operator not known")
            };
            let mut set_1 = self.values.clone();
            set_1.insert(operation.part_id, r1);
            let mut set_2 = self.values.clone();
            set_2.insert(operation.part_id, r2);
            return vec![TheoreticalPart { values: set_1 }, TheoreticalPart { values: set_2 }]
        }

        panic!("Help");
    }

    fn overlap(&self, other: &TheoreticalPart) -> bool {
        let keys = vec!['x','m', 'a', 's'];

        for key in keys {
            let r1 = self.values.get(&key).unwrap();
            let r2 = other.values.get(&key).unwrap();

            if !r1.contains(&r2.start) || r1.contains(&r2.end) {
                return false;
            }
        }

        return true
    }

    fn sum(&self) -> i32 {
        self.values.iter().map(|f| (f.1.end - f.1.start).abs()).sum::<i32>()
    }

     fn get_combos(&self) -> u128 {
         let mut combination:u128 = 1;
         for (id, range) in &self.values {
             combination *= (range.end - range.start + 1).abs() as u128;
         }

         combination
     }

    fn print(&self) {
        println!("x {}-{} m {}-{} a {}-{} s {}-{}", self.values.get(&'x').unwrap().start, self.values.get(&'x').unwrap().end,
                 self.values.get(&'m').unwrap().start, self.values.get(&'m').unwrap().end,
                 self.values.get(&'a').unwrap().start, self.values.get(&'a').unwrap().end,
                 self.values.get(&'s').unwrap().start, self.values.get(&'s').unwrap().end);
    }
}

struct Part {
    values: HashMap<char, i32>
}

impl Part {
    fn parse(input_line: &str) -> Self {
        let mut values:HashMap<char, i32> = HashMap::new();
        input_line.replace("{", "").replace("}", "").split(",").for_each(|split| {
            let mut part_split = split.split("=");
            let part_char = part_split.nth(0).unwrap();
            let part_value = parse_numbers(part_split.nth(0).unwrap()).unwrap().1;

            values.insert(part_char.chars().next().unwrap(), part_value);
        });

        Part { values }
    }

    fn sum(&self) -> i32 {
        self.values.iter().map(|(id, val)| *val).sum::<i32>()
    }
}

struct WorkflowRule {
    requirement: Option<Operation>,
    destination: String
}
impl WorkflowRule {
    fn parse(input_line: &str) -> IResult<&str, Self> {
        if(input_line.contains(":")){
            let (input_line, id) = take_till(|f| f == '<' || f == '>')(input_line)?;
            let (input_line, operator) = take(1usize)(input_line)?;
            let (input_line, requirement) = take_until(":")(input_line)?;
            let (input_line, _) = tag(":")(input_line)?;
            let destination = input_line;

            return Ok((input_line, WorkflowRule {destination: destination.to_string(), requirement: Some(Operation { operator: operator.chars().next().unwrap(), part_req: parse_numbers(requirement).unwrap().1, part_id: id.chars().next().unwrap() })}))
        }


        Ok((input_line, WorkflowRule { destination: input_line.to_string(), requirement: None}))
    }
    fn can_apply(&self, part: &Part) -> bool {
        if self.requirement.is_none() {
            return true;
        }

        self.requirement.as_ref().unwrap().evaluate(part)
    }

    fn can_apply_in_theory(&self, part: &TheoreticalPart) -> bool {
        if self.requirement.is_none() {
            return true;
        }

        self.requirement.as_ref().unwrap().evaluate_in_theory(part)
    }

}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Operation {
    part_id: char,
    part_req: i32,
    operator: char
}

impl Operation {
    fn evaluate(&self, part: &Part) -> bool {
        if let Some(val) = part.values.get(&self.part_id) {
            return match self.operator {
                '<' => val < &self.part_req,
                '>' => val > &self.part_req,
                _ => panic!("operator not known")
            }
        }

        panic!("Unknown part type")
    }

    fn evaluate_in_theory(&self, part: &TheoreticalPart) -> bool {
        if let Some(val) = part.values.get(&self.part_id) {

            return match self.operator {
                '<' => val.start < self.part_req,
                '>' => val.end > self.part_req,
                _ => panic!("operator not known")
            }
        }

        panic!("Unknown part type")
    }

}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;
    use crate::days::day_19::{factory_line, factory_line_2, Operation, parse_information, Part, TheoreticalPart, Workflow};

    #[test]
    fn can_parse_workflow() -> Result<(), String> {
        let input = r#"px{a<2006:qkq,m>2090:A,rfg}"#;

        let workflow = Workflow::parse(input).unwrap().1;

        assert_eq!(workflow.name, "px");
        assert_eq!(workflow.rules.len(), 3);
        assert_eq!(workflow.rules[0].destination, "qkq");
        assert_eq!(workflow.rules[1].requirement.unwrap().part_id, 'm');
        assert_eq!(workflow.rules[1].requirement.unwrap().part_req, 2090);
        assert_eq!(workflow.rules[2].requirement.is_none(), true);
        assert_eq!(workflow.rules[2].destination, "rfg");

        Ok(())
    }

    #[test]
    fn can_get_workflow_destination_for_part() {
        let workflow = Workflow::parse(r#"px{a<2006:qkq,m>2090:A,rfg}"#).unwrap().1;
        let part = Part::parse(r#"{x=787,m=2655,a=1222,s=2876}"#);

        let destination = workflow.get_part_destination(&part);

        assert_eq!(destination, "qkq");
    }

    #[test]
    fn can_parse_part() {
        let input = r#"{x=787,m=2655,a=1222,s=2876}"#;

        let part = Part::parse(input);

        assert_eq!(part.values.len(), 4);
        assert_eq!(part.values.get(&'x').unwrap(), &787);
        assert_eq!(part.values.get(&'a').unwrap(), &1222);
    }

    #[test]
    fn can_parse_full_input() {
        let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

        let (workflows, parts) = parse_information(input.to_string());

        assert_eq!(workflows.len(), 11);
        assert_eq!(parts.len(), 5);
    }

    #[test]
    fn factory_line_runs_well() {
        let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

        let result = factory_line(input.to_string());

        assert_eq!(result, 19114);
    }

    #[test]
    fn theoryitical_part_can_be_split() {
        let operation = Operation { part_id: 'x', part_req: 2000, operator: '<'};
        let start_part = TheoreticalPart::blank();

        let split = start_part.split_part(operation);

        assert_eq!(split.len(), 2);
        assert_eq!(split[0].values.get(&'x').unwrap().end, 1999);
        assert_eq!(split[1].values.get(&'x').unwrap().start, 2000);
    }

    #[test]
    fn can_get_workflow_destination_for_theoritical_parts() {
        let workflow = Workflow::parse(r#"px{a<2006:qkq,m>2090:A,rfg}"#).unwrap().1;
        let part = TheoreticalPart::blank();

        let results = workflow.process_theoretical_part(&part);

        for result in &results {
            result.0.print();
        }

        assert_eq!(results.len(), 3);
    }

    #[test]
    fn can_get_only_split_destination_for_theoritical_parts() {
        let workflow = Workflow::parse(r#"in{s<1351:px,qqz}"#).unwrap().1;
        let part = TheoreticalPart::blank();

        let results = workflow.process_theoretical_part(&part);

        for result in &results {
            result.0.print();
        }

        assert_eq!(results.len(), 2);
    }


    #[test]
    fn factory_line_2_runs_well() {
        let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

        let result = factory_line_2(input.to_string());

        assert_eq!(result, 167409079868000);
    }


}