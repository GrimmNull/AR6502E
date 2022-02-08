
pub mod reducer_type {
    use crate::action_types::action_types::ActionTypes;
    use crate::action::action_type::Action;
    use crate::state::state_type::State;
    use std::i64;

    pub fn reducer(mut state: State, action: &Action) -> State {
        return match action.action_type {
            ActionTypes::ADC => {
                // Parsing the operands
                let first_operand = i64::from_str_radix(&action.action_arg1.to_string()[1..action.action_arg1.len()], 16).unwrap();
                let second_operand = if action.action_arg2.to_string() == "X" || action.action_arg2.to_string() == "Y" {if action.action_arg2.to_string() == "X" {state.x as u8} else {state.y as u8}} else { action.action_arg2.parse::<u8>().unwrap() };

                // Checking if an overflow will occur and toggle the flag
                if (state.memory[first_operand as usize] as u16 + second_operand as u16) > 256 {
                    state.c = true;
                }

                // Add the numbers and return the state
                state.memory[first_operand as usize] = state.memory[first_operand as usize].wrapping_add(second_operand);
                state.pc+= 1;
                state
            },
            ActionTypes::AND => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::ASL => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::BCC => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::BCS => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::BEQ => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::BIT => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::BMI => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::BNE => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::BPL => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::BRK => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::BVC => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::BVS => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::CLC => {
                state.c = false;
                state.pc+= 1;
                state
            },
            ActionTypes::CLD => {
                state.d= false;
                state.pc+= 1;
                state
            },
            ActionTypes::CLI => {
                state.i = false;
                state.pc+= 1;
                state
            },
            ActionTypes::CLV => {
                state.v = false;
                state.pc+= 1;
                state
            },
            ActionTypes::CMP => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::CPX => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::CPY => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::DEC => {
                let first_operand = i64::from_str_radix(&action.action_arg1.to_string()[1..action.action_arg1.len()], 16).unwrap();
                state.memory[first_operand as usize]-=1;
                state.pc+= 1;
                state
            },
            ActionTypes::DEX => {
                state.x-=1;
                state.pc+= 1;
                state
            },
            ActionTypes::DEY => {
                state.y-=1;
                state.pc+= 1;
                state
            },
            ActionTypes::EOR => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::INC => {
                let first_operand = i64::from_str_radix(&action.action_arg1.to_string()[1..action.action_arg1.len()], 16).unwrap();
                state.memory[first_operand as usize]+=1;
                state.pc+= 1;
                state
            },
            ActionTypes::INX => {
                state.x+=1;
                state
            },
            ActionTypes::INY => {
                state.y+=1;
                state
            },
            ActionTypes::JMP => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::JSR => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::LDA => {
                let first_operand = i64::from_str_radix(&action.action_arg1.to_string()[1..action.action_arg1.len()], 16).unwrap();
                state.memory[first_operand as usize]= if action.action_arg1.to_string() == "X" {state.x as u8} else {state.y as u8};
                state.pc+= 1;
                state
            },
            ActionTypes::LDX => {
                if action.action_arg1.to_string() == "Y" {
                    state.x = state.y;
                } else {
                    let first_operand = i64::from_str_radix(&action.action_arg1.to_string()[1..action.action_arg1.len()], 16).unwrap();
                    state.x = state.memory[first_operand as usize] as i8;
                }
                state.pc+= 1;
                state
            },
            ActionTypes::LDY => {
                if action.action_arg1.to_string() == "X" {
                    state.y = state.x;
                } else {
                    let first_operand = i64::from_str_radix(&action.action_arg1.to_string()[1..action.action_arg1.len()], 16).unwrap();
                    state.y = state.memory[first_operand as usize] as i8;
                }
                state.pc+= 1;
                state
            },
            ActionTypes::LSR => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::NOP => {
                state.pc+= 1;
                state
            },
            ActionTypes::ORA => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::PHA => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::PHP => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::PLA => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::PLP => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::ROL => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::ROR => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::RTI => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::RTS => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::SBC => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::SEC => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::SED => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::SEI => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::STA => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::STX => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::STY => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::TAX => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::TAY => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::TSX => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::TXA => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::TXS => {
                println!("This action wasn't implemented yet");
                state
            },
            ActionTypes::TYA => {
                println!("This action wasn't implemented yet");
                state
            }
        }
    }

}