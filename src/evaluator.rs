/**
 * Converts a hexadecimal string to a vector of u8 bytes.
 *
 * @param {string} hex_string - The hexadecimal string to convert.
 * @returns {Result<Vec<u8>, std::num::ParseIntError>} - A Result containing the converted bytes or an error.
 */
pub fn hex_string_to_bytes(hex_string: &str) -> Result<Vec<&str>, std::num::ParseIntError> {

    let mut bytes = Vec::new();

    for i in (0..hex_string.len()).step_by(2) {
        bytes.push(&hex_string[i..i + 2]);
    }

    Ok(bytes)

}

/**
 * Calculates the next pointer value based on the current pointer, sum, and a flag indicating if it's the last index.
 *
 * @param {u32} pointer - The current pointer value.
 * @param {u32} sum - The sum value used for calculating the next pointer.
 * @param {boolean} last_index - A flag indicating if it's the last index.
 * @returns {u32} - The next pointer value.
 */
pub fn set_pointer(pointer: u32, input_qty: &Option<Vec<u32>>, last_index: bool) -> u32 {

    let inputs: &Option<Vec<u32>> = input_qty;
    let new_pointer;

    if let Some(_input) = &input_qty {

        let sum: u32 = inputs.as_ref().unwrap().iter().sum();
        if last_index {
            // dont move pointer on last opcode
            new_pointer = pointer
        } else if sum > 0 {
            //+1 to be one ahead of skipped byte
            new_pointer = (pointer + sum) + 1
        } else {
            //60 80 60 40 - I need two to get to the next 60 `PUSH1`
            new_pointer = pointer + 1
        }

    } else {

        new_pointer = pointer + 1;

    }

    new_pointer
}

/**
 * Finds stack inputs based on the index, input quantity, and parsed bytes.
 *
 * @param {number} index - The index to start searching for stack inputs.
 * @param {Vec<u32>} input_qty - An array of input quantities.
 * @param {Vec<u8>} parsed_bytes - An array of parsed bytes.
 * @returns {Vec<string>} - An array of strings representing stack inputs.
 */
pub fn find_stack_inputs(index: usize, input_qty: &Option<Vec<u32>>, parsed_bytes: &Vec<&str>) -> Vec<String> {

    let mut arg_counter = index;
    let mut temp_sequence: Vec<String> = vec![];

    for  (_i, byte_size) in input_qty.as_ref().unwrap().iter().enumerate() {

        let stringifies_slice: &Vec<String> = &parsed_bytes[arg_counter+1..arg_counter+1+*byte_size as usize]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let joined_slice = stringifies_slice.concat().clone();
        temp_sequence.push(joined_slice);
        arg_counter = arg_counter + *byte_size as usize;

    }

    temp_sequence

}