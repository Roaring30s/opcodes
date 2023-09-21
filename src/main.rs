mod data;
mod evaluator;

fn main() {
    // Inputs
    let bytecode_hex = "608060405234801561001057600080fd5b50610173806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c80636d4ce63c14610030575b600080fd5b61003861004e565b604051610045919061011b565b60405180910390f35b60606040518060400160405280600b81526020017f48656c6c6f20576f726c64000000000000000000000000000000000000000000815250905090565b600081519050919050565b600082825260208201905092915050565b60005b838110156100c55780820151818401526020810190506100aa565b60008484015250505050565b6000601f19601f8301169050919050565b60006100ed8261008b565b6100f78185610096565b93506101078185602086016100a7565b610110816100d1565b840191505092915050565b6000602082019050818103600083015261013581846100e2565b90509291505056fea2646970667358221220e775ca7f46a8341e4bbf54b44465b819e72ef5db2dc90c16818f11455ae149ec64736f6c63430008120033";
    // Output
    let mut opcode_sequence: Vec<String> = vec![];

    // Initiate before Evaluation
    let mut pointer: u32 = 0;
    let opcode_map = data::initiate_opcode_map();
    let parsed_bytes = evaluator::hex_string_to_bytes(bytecode_hex).unwrap();

    // Evaluate bytecode
    for (index, byte) in parsed_bytes.iter().enumerate() {
        if index == parsed_bytes.len()-1 { // Last opcode in bytecode

            // Lookup in map and add opcode to sequence
            opcode_sequence.push(opcode_map[byte].opcode.clone());
            // Set location for pointer to find next opcode within bytecode
            pointer = evaluator::set_pointer(
                pointer,
                &opcode_map[byte].input_qty,
                true
            );

        } else if index == 0 { // First opcode in bytecode

            // Lookup in map and add opcode to sequence
            opcode_sequence.push(opcode_map[byte].opcode.clone());
            // Set location for pointer to find next opcode within bytecode
            pointer = evaluator::set_pointer(
                pointer,
                &opcode_map[byte].input_qty,
                false
            );

            // Get stack inputs placed next to collected opcode
            let mut temp_sequence: Vec<String>;
            if let Some(_input) = &opcode_map[byte].input_qty {
                temp_sequence = evaluator::find_stack_inputs(
                    index,
                    &opcode_map[byte].input_qty,
                    &parsed_bytes
                );
                opcode_sequence.append(&mut temp_sequence);
            }
            // If pointer exceeds total length of bytecode, halt execution
            if pointer as usize > parsed_bytes.len()-1 {
                break;
            }

        } else if index as u32 == pointer { // When index == pointer, we've hit another opcode

            // Lookup in map and add opcode to sequence
            opcode_sequence.push(opcode_map[byte].opcode.clone());

            // Set location for pointer to find next opcode within bytecode
            pointer = evaluator::set_pointer(
                pointer,
                &opcode_map[byte].input_qty,
                false
            );

            // Get stack inputs placed next to collected opcode
            let mut temp_sequence: Vec<String>;
            if let Some(_input) = &opcode_map[byte].input_qty {
                temp_sequence = evaluator::find_stack_inputs(
                    index,
                    &opcode_map[byte].input_qty,
                    &parsed_bytes
                );
                opcode_sequence.append(&mut temp_sequence);
            }

            // If pointer exceeds total length of bytecode, halt execution
            if pointer as usize > parsed_bytes.len()-1 {
                break;
            }

        }
    }
    dbg!(opcode_sequence);

}

// Notes
// virtual input stack and virtual output stack to track values to be manipulated with.
