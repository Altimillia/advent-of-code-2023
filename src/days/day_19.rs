use std::collections::HashMap;
use std::fmt::Display;
use nom::bytes::complete::{tag, take, take_till, take_until, take_while_m_n};
use nom::IResult;
use std::cmp;
use crate::tools::parse_numbers;
pub fn part_one(input: String) -> impl Display {
    0
}

// fn factory_line(input: String) -> i32 {
//
// }
//
// fn parse_information(input: String) -> (HashMap<String, Workflow>, Vec<Part>) {
//
// }

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
        let (input_line, rules_string) = take_until("}")(input_line)?;

        let rules = rules_string.split(",").map(|rule| WorkflowRule::parse(rule).unwrap().1).collect();

        Ok((input_line, Workflow { name: name.to_string(), rules }))
    }
}

struct Part {
    values: HashMap<char, i32>
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
            let (input_line, _) = tag(":")(input_line)?;
            let destination = input_line;

            return Ok((input_line, WorkflowRule {destination: destination.to_string(), requirement: Some(Operation { operator: operator.chars().next().unwrap(), part_req: 0, part_id: id.chars().next().unwrap() })}))
        }


        Ok((input_line, WorkflowRule { destination: input_line.to_string(), requirement: None}))
    }
    fn can_apply(&self, part: Part) -> bool {
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
    fn evaluate(&self, part: Part) -> bool {
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
    use crate::days::day_19::Workflow;

    #[test]
    fn can_parse_workflow() -> Result<(), String> {
        let input = r#"px{a<2006:qkq,m>2090:A,rfg}"#;

        let workflow = Workflow::parse(input).unwrap().1;

        assert_eq!(workflow.name, "px");
        assert_eq!(workflow.rules.len(), 3);
        assert_eq!(workflow.rules[0].destination, "qkq");
        assert_eq!(workflow.rules[1].requirement.unwrap().part_id, 'm');

        Ok(())
    }
}