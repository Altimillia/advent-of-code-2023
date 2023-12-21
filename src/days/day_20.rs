use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::mem;
use num::bigint::Sign;
use num::integer::gcd;

pub fn part_one(input: String) -> impl Display {
    let mut signal_map = parse_information(input);
    press_the_button(&mut signal_map, 1000)
}

pub fn part_two(input: String) -> impl Display {
    let mut signal_map = parse_information(input);
    press_until_signal_received(&mut signal_map)
}



fn press_the_button(signal_map: &mut HashMap<String, Box<dyn Module>>, iterations: i32) -> u64 {
    let mut signal_queue:VecDeque<Signal> = VecDeque::new();
    let mut low_counter = 0;
    let mut high_counter = 0;

    for i in 0..iterations {
        signal_queue.push_back(Signal::Low("button".to_string(), "broadcaster".to_string()));
        while signal_queue.len() > 0 {
            let next_signal = signal_queue.pop_front().unwrap();

            match &next_signal {
                Signal::High(_, dest) => {
                    high_counter += 1;
                    if let Some(destination_module) = signal_map.get_mut(&dest.clone()) {
                        let new_signals = destination_module.receive_signal(next_signal.clone());
                        for new_signal in new_signals {
                            signal_queue.push_back(new_signal);
                        }
                    }
                }
                Signal::Low(_, dest) => {
                    low_counter += 1;
                    if let Some(destination_module) = signal_map.get_mut(&dest.clone()) {
                        let new_signals = destination_module.receive_signal(next_signal.clone());
                        for new_signal in new_signals {
                            signal_queue.push_back(new_signal);
                        }
                    }
                }
            }
        }
    }

    low_counter * high_counter

}



fn press_until_signal_received(signal_map: &mut HashMap<String, Box<dyn Module>>) -> u64 {
    let mut signal_queue:VecDeque<Signal> = VecDeque::new();
    let mut button_counter = 0;
    let mut rx_node_sender_map:HashMap<String, u64> = HashMap::new();

    while rx_node_sender_map.len() < 4 {

        button_counter += 1;
        signal_queue.push_back(Signal::Low("button".to_string(), "broadcaster".to_string()));
        while signal_queue.len() > 0 {
            let next_signal = signal_queue.pop_front().unwrap();

            match &next_signal {
                Signal::High(src, dest) => {
                    if dest == &"zh".to_string() {
                        rx_node_sender_map.insert(src.to_string(), button_counter);
                    }
                    if let Some(destination_module) = signal_map.get_mut(&dest.clone()) {
                        let new_signals = destination_module.receive_signal(next_signal.clone());
                        for new_signal in new_signals {
                            signal_queue.push_back(new_signal);
                        }
                    }
                }
                Signal::Low(_, dest) => {
                    if let Some(destination_module) = signal_map.get_mut(&dest.clone()) {
                        let new_signals = destination_module.receive_signal(next_signal.clone());
                        for new_signal in new_signals {
                            signal_queue.push_back(new_signal);
                        }
                    }
                }
            }
        }
    }
    let value_set:Vec<u64> = rx_node_sender_map.values().map(|value| *value).collect();
    find_lcm_of_set(value_set)
}

fn find_lcm_of_set(numbers: Vec<u64>) -> u64 {
    numbers.iter().cloned().fold(1, |acc, x| lcm(acc, x))
}
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn parse_information(input: String) -> HashMap<String, Box<dyn Module>> {
    let mut signal_map:HashMap<String, Box<dyn Module>> = HashMap::new();
    input.lines().for_each(|line|
        {
            let module = create_module(line);
            signal_map.insert(module.get_id(), create_module(line));
        }
    );

    let mut destination_map:HashMap<String, Vec<String>> = HashMap::new();

    signal_map.iter().for_each(|(key, module)| {
        destination_map.insert(module.get_id(), vec![]);
    });

    signal_map.iter().for_each(|(key, module)| {
        module.get_destinations().iter().for_each(|dest| {

            if let Some(destinations) = destination_map.get_mut(dest) {
                destinations.push(module.get_id());
            }
        });
    });

    signal_map.iter_mut().for_each(|(key, module)| {
        let destination_list = destination_map.get(key).unwrap();
        module.initialize(destination_list.clone());
    });

    signal_map
}

fn create_module(input_line: &str) -> Box<dyn Module> {
    let mut split = input_line.split("->");
    let module_id = split.nth(0).unwrap();
    let destinations:Vec<String> = split.nth(0).unwrap().split(",").map(|m| m.trim().to_string()).collect();

    let mut chars = module_id.chars();
    let module_type = chars.next().unwrap();
    let module_identifier = chars.as_str().trim();

    return match module_type {
        '&' => Box::new(ConjunctionModule { id: module_identifier.to_string(), last_pulse: Signal::Low("".to_string(), "".to_string()), connected_to: destinations, memory:HashMap::new() }),
        '%' => Box::new(FlipFlopModule { id: module_identifier.to_string(), on: false, connected_to: destinations }),
        _ => Box::new(BroadcasterModule { id: module_id.trim().to_string(), connected_to: destinations })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
enum Signal {
    High(String, String),
    Low(String, String)
}

trait Module {
    fn receive_signal(&mut self, signal: Signal) -> Vec<Signal>;
    fn get_destinations(&self) -> Vec<String>;
    fn get_id(&self) -> String;
    fn initialize(&mut self, inputs: Vec<String>);
}

struct BroadcasterModule {
    id: String,
    connected_to: Vec<String>
}
impl Module for BroadcasterModule {
    fn receive_signal(&mut self, signal: Signal) -> Vec<Signal> {
        return match signal {
            Signal::High(_, _) => self.connected_to.iter().map(|dest| return Signal::High(self.id.clone(), dest.clone())).collect(),
            Signal::Low(_, _) => self.connected_to.iter().map(|dest| return Signal::Low(self.id.clone(), dest.clone())).collect(),
        }
    }

    fn get_destinations(&self) -> Vec<String> {
        return self.connected_to.clone();
    }

    fn get_id(&self) -> String {
        return self.id.clone();
    }

    fn initialize(&mut self, inputs: Vec<String>) {

    }

}

struct ConjunctionModule {
    id: String,
    last_pulse: Signal,
    connected_to: Vec<String>,
    memory: HashMap<String, Signal>
}

impl Module for ConjunctionModule {
    fn receive_signal(&mut self, signal: Signal) -> Vec<Signal> {
        match &signal {
            Signal::High(src, _) => {
                self.memory.insert(src.clone(), signal.clone());

            }
            Signal::Low(src, _) => {
                self.memory.insert(src.clone(), signal.clone());
            }
        }

        return if self.memory.values().all(|s| mem::discriminant(s) == mem::discriminant(&Signal::High(String::new(), String::new()))) {
            self.connected_to.iter().map(|dest| return Signal::Low(self.id.clone(), dest.clone())).collect()
        } else {
            let id = self.id.as_str();
            self.connected_to.iter().map(|dest| return Signal::High(self.id.clone(), dest.clone())).collect()
        }
    }

    fn get_destinations(&self) -> Vec<String> {
        return self.connected_to.clone();
    }

    fn get_id(&self) -> String {
        return self.id.clone();
    }

    fn initialize(&mut self, inputs: Vec<String>) {
        for input in inputs {
            self.memory.insert(input.clone(), Signal::Low(input.clone(), self.id.clone()));
        }
    }
}

struct FlipFlopModule {
    id: String,
    on: bool,
    connected_to: Vec<String>
}

impl Module for FlipFlopModule {
    fn receive_signal(&mut self, signal: Signal) -> Vec<Signal> {
        return match signal {
            Signal::High(_, _) => vec![],
            Signal::Low(_, _) => {
                if self.on {
                    self.on = false;
                    self.connected_to.iter().map(|dest| return Signal::Low(self.id.clone(), dest.clone())).collect()
                }
                else {
                    self.on = true;
                    self.connected_to.iter().map(|dest| return Signal::High(self.id.clone(), dest.clone())).collect()
                }
            }
        }
    }

    fn get_destinations(&self) -> Vec<String> {
        return self.connected_to.clone();
    }

    fn get_id(&self) -> String {
        return self.id.clone();
    }

    fn initialize(&mut self, inputs: Vec<String>) {

    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_20::{create_module, parse_information, press_the_button, Signal};

    #[test]
    fn can_create_module_from_line() {
        let input = r#"&zp -> px, gp, cl, bh, fn, ls, hs"#;

        let result = create_module(input);

        assert_eq!(result.get_destinations(), vec!["px", "gp", "cl", "bh", "fn", "ls", "hs"]);
        assert_eq!(result.get_id(), "zp");
    }

    #[test]
    fn can_send_broadcast_signal() {
        let input = r#"broadcaster -> ls, bv, dc, br"#;
        let mut module = create_module(input);

        let expected = vec![Signal::Low("broadcaster".to_string(), "ls".to_string()),
                            Signal::Low("broadcaster".to_string(), "bv".to_string()),
                            Signal::Low("broadcaster".to_string(), "dc".to_string()),
                            Signal::Low("broadcaster".to_string(), "br".to_string())];

        let result = module.receive_signal(Signal::Low("".to_string(),"".to_string()));

        assert_eq!(result[0], expected[0]);
        assert_eq!(result.len(), expected.len());
    }

    #[test]
    fn flip_flop_returns_nothing_on_high_signal() {
        let input = r#"%a -> b"#;
        let mut module = create_module(input);

        let result = module.receive_signal(Signal::High("".to_string(),"".to_string()));
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn flip_flop_flips_when_low() {
        let input = r#"%a -> b"#;
        let mut module = create_module(input);

        let result = module.receive_signal(Signal::Low("".to_string(),"".to_string()));

        assert_eq!(result.len(), 1);
        let expected = Signal::High("a".to_string(), "b".to_string());
        assert!(matches!(&result[0], expected));
        let result = module.receive_signal(Signal::Low("".to_string(),"".to_string()));
        assert_eq!(result.len(), 1);
        let expected = Signal::Low("a".to_string(), "b".to_string());
        assert!(matches!(&result[0], expected));
    }

    #[test]
    fn conjuction_module_sends_signal() {
        let input = r#"&inv -> a"#;
        let mut module = create_module(input);

        let result = module.receive_signal(Signal::Low("c".to_string(), "".to_string()));
        assert_eq!(result.len(), 1);
        let expected = Signal::High("inv".to_string(), "a".to_string());

        let match_result = match &result[0] {
            Signal::High(_, _) => true,
            Signal::Low(_, _) => false
        };

        assert_eq!(match_result, true);
    }

    #[test]
    fn conjuction_module_remembers_input() {
        let input = r#"&inv -> a"#;
        let mut module = create_module(input);

        let result = module.receive_signal(Signal::High("c".to_string(), "".to_string()));
        assert_eq!(result.len(), 1);
        let expected = Signal::Low("inv".to_string(), "a".to_string());
        let match_result = match &result[0] {
            Signal::High(_, _) => false,
            Signal::Low(_, _) => true
        };

        assert_eq!(match_result, true);

    }

    #[test]
    fn conjuction_module_remembers_initalized_input() {
        let input = r#"&con -> output"#;
        let mut module = create_module(input);
        module.initialize(vec!["a".to_string(), "b".to_string()]);

        let result = module.receive_signal(Signal::High("a".to_string(), "".to_string()));
        assert_eq!(result.len(), 1);

        let match_result = match &result[0] {
            Signal::High(_, _) => true,
            Signal::Low(_, _) => false
        };

        assert_eq!(match_result, true);
    }

    #[test]
    fn basic_signal_test_gets_correct_single_result() {
        let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

        let mut map = parse_information(input.to_string());

        let result = press_the_button(&mut map, 1);

        assert_eq!(result, 8 * 4);
    }

    #[test]
    fn basic_signal_test_1000_times() {
        let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

        let mut map = parse_information(input.to_string());

        let result = press_the_button(&mut map, 1000);

        assert_eq!(result, 32000000);
    }

    #[test]
    fn advanced_signal_test_1000_times() {
        let input = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

        let mut map = parse_information(input.to_string());

        let result = press_the_button(&mut map, 1000);

        assert_eq!(result, 11687500);
    }
}