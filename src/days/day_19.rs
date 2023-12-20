use std::collections::HashMap;
use std::fmt::Display;
use nom::bytes::complete::{tag, take, take_till, take_until, take_while_m_n};
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
    0
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
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;
    use crate::days::day_19::{factory_line, parse_information, Part, Workflow};

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
}