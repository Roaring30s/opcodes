use std::collections::HashMap;
#[derive(Debug)]
pub struct BytecodeData {
    pub opcode: String,
    pub input_qty: Option<Vec<u32>>,
    pub output_qty: i32,
}

/**
 * Initializes and returns a HashMap<u8, BytecodeData> containing predefined opcode information.
 *
 * @returns {HashMap<u8, BytecodeData>} A HashMap containing opcode mappings where each opcode value (u8)
 * is associated with a BytecodeData struct describing the opcode.
 *
 * @example
 * // Define a struct to represent opcode data
 * struct BytecodeData {
 *     opcode: string,
 *     input_qty: Array<number>,
 *     output_qty: number,
 * }
 *
 * // Initialize the opcode map
 * let opcode_map = initiate_opcode_map();
 *
 * @note
 * This function initializes the opcode map with predefined opcodes and their associated
 * information. You can extend the function to include more opcodes or customize the
 * opcode data as needed.
 */
pub fn initiate_opcode_map() -> HashMap<&'static str, BytecodeData> {

    let mut opcode_map: HashMap<&str, BytecodeData> = HashMap::new();

    // load opcode map

    opcode_map.insert("00", BytecodeData {
        opcode: "STOP".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("01", BytecodeData {
        opcode: "ADD".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("02", BytecodeData {
        opcode: "MUL".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("03", BytecodeData {
        opcode: "SUB".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("04", BytecodeData {
        opcode: "DIV".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("05", BytecodeData {
        opcode: "SDIV".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("06", BytecodeData {
        opcode: "MOD".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("07", BytecodeData {
        opcode: "SMOD".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("08", BytecodeData {
        opcode: "ADDMOD".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("09", BytecodeData {
        opcode: "MULMOD".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("0a", BytecodeData {
        opcode: "EXP".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("0b", BytecodeData {
        opcode: "SIGNEXTEND".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("10", BytecodeData {
        opcode: "LT".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("11", BytecodeData {
        opcode: "GT".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("12", BytecodeData {
        opcode: "SLT".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("13", BytecodeData {
        opcode: "SGT".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("14", BytecodeData {
        opcode: "EQ".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("15", BytecodeData {
        opcode: "ISZERO".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("16", BytecodeData {
        opcode: "AND".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("17", BytecodeData {
        opcode: "OR".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("18", BytecodeData {
        opcode: "XOR".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("19", BytecodeData {
        opcode: "NOT".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("1a", BytecodeData {
        opcode: "BYTE".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("1b", BytecodeData {
        opcode: "SHL".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("1c", BytecodeData {
        opcode: "SHR".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("1d", BytecodeData {
        opcode: "SAR".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("20", BytecodeData {
        opcode: "SHA3".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("30", BytecodeData {
        opcode: "ADDRESS".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("31", BytecodeData {
        opcode: "BALANCE".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("32", BytecodeData {
        opcode: "ORIGIN".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("33", BytecodeData {
        opcode: "CALLER".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("34", BytecodeData {
        opcode: "CALLVALUE".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("35", BytecodeData {
        opcode: "CALLDATALOAD".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("36", BytecodeData {
        opcode: "CALLDATASIZE".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("37", BytecodeData {
        opcode: "CALLDATACOPY".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("38", BytecodeData {
        opcode: "CODESIZE".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("39", BytecodeData {
        opcode: "CODECOPY".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("3a", BytecodeData {
        opcode: "GASPRICE".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("3b", BytecodeData {
        opcode: "EXTCODESIZE".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("3c", BytecodeData {
        opcode: "EXTCODECOPY".to_string(),
        input_qty: None,
        output_qty: 0,
    });


    opcode_map.insert("3d", BytecodeData {
        opcode: "RETURNDATASIZE".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("3e", BytecodeData {
        opcode: "RETURNDATACOPY".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("3f", BytecodeData {
        opcode: "EXTCODEHASH".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("40", BytecodeData {
        opcode: "BLOCKHASH".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("41", BytecodeData {
        opcode: "COINBASE".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("42", BytecodeData {
        opcode: "TIMESTAMP".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("43", BytecodeData {
        opcode: "NUMBER".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("44", BytecodeData {
        opcode: "PREVRANDAO".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("45", BytecodeData {
        opcode: "GASLIMIT".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("46", BytecodeData {
        opcode: "CHAINID".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("47", BytecodeData {
        opcode: "SELFBALANCE".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("48", BytecodeData {
        opcode: "BASEFEE".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("50", BytecodeData {
        opcode: "POP".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("51", BytecodeData {
        opcode: "MLOAD".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("52", BytecodeData {
        opcode: "MSTORE".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("53", BytecodeData {
        opcode: "MSTORE8".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("54", BytecodeData {
        opcode: "SLOAD".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("55", BytecodeData {
        opcode: "SSTORE".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("56", BytecodeData {
        opcode: "JUMP".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("57", BytecodeData {
        opcode: "JUMPI".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("58", BytecodeData {
        opcode: "PC".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("59", BytecodeData {
        opcode: "MSIZE".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("5a", BytecodeData {
        opcode: "GAS".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("5b", BytecodeData {
        opcode: "JUMPDEST".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("5f", BytecodeData {
        opcode: "PUSH0".to_string(),
        input_qty: None,
        output_qty: 0,
    });

    opcode_map.insert("60", BytecodeData {
        opcode: "PUSH1".to_string(),
        input_qty: Some(vec![1]),
        output_qty: 0,
    });

    opcode_map.insert("61", BytecodeData {
        opcode: "PUSH2".to_string(),
        input_qty: Some(vec![2]),
        output_qty: 0,
    });

    opcode_map.insert("62", BytecodeData {
        opcode: "PUSH3".to_string(),
        input_qty: Some(vec![3]),
        output_qty: 0,
    });

    opcode_map.insert("63", BytecodeData {
        opcode: "PUSH4".to_string(),
        input_qty: Some(vec![4]),
        output_qty: 0,
    });

    opcode_map.insert("64", BytecodeData {
        opcode: "PUSH5".to_string(),
        input_qty: Some(vec![5]),
        output_qty: 0,
    });

    opcode_map.insert("65", BytecodeData {
        opcode: "PUSH6".to_string(),
        input_qty: Some(vec![6]),
        output_qty: 0,
    });

    opcode_map.insert("66", BytecodeData {
        opcode: "PUSH7".to_string(),
        input_qty: Some(vec![7]),
        output_qty: 0,
    });

    opcode_map.insert("67", BytecodeData {
        opcode: "PUSH8".to_string(),
        input_qty: Some(vec![8]),
        output_qty: 0,
    });

    opcode_map.insert("68", BytecodeData {
        opcode: "PUSH9".to_string(),
        input_qty: Some(vec![9]),
        output_qty: 0,
    });

    opcode_map.insert("69", BytecodeData {
        opcode: "PUSH10".to_string(),
        input_qty: Some(vec![10]),
        output_qty: 0,
    });

    opcode_map.insert("6a", BytecodeData {
        opcode: "PUSH11".to_string(),
        input_qty: Some(vec![11]),
        output_qty: 0,
    });

    opcode_map.insert("6b", BytecodeData {
        opcode: "PUSH12".to_string(),
        input_qty: Some(vec![12]),
        output_qty: 0,
    });

    opcode_map.insert("6c", BytecodeData {
        opcode: "PUSH13".to_string(),
        input_qty: Some(vec![13]),
        output_qty: 0,
    });

    opcode_map.insert("6d", BytecodeData {
        opcode: "PUSH14".to_string(),
        input_qty: Some(vec![14]),
        output_qty: 0,
    });

    opcode_map.insert("6e", BytecodeData {
        opcode: "PUSH15".to_string(),
        input_qty: Some(vec![15]),
        output_qty: 0,
    });

    opcode_map.insert("6f", BytecodeData {
        opcode: "PUSH16".to_string(),
        input_qty: Some(vec![16]),
        output_qty: 0,
    });

    opcode_map.insert("70", BytecodeData {
        opcode: "PUSH17".to_string(),
        input_qty: Some(vec![17]),
        output_qty: 0,
    });

    opcode_map.insert("71", BytecodeData {
        opcode: "PUSH18".to_string(),
        input_qty: Some(vec![18]),
        output_qty: 0,
    });

    opcode_map.insert("72", BytecodeData {
        opcode: "PUSH19".to_string(),
        input_qty: Some(vec![19]),
        output_qty: 0,
    });

    opcode_map.insert("73", BytecodeData {
        opcode: "PUSH20".to_string(),
        input_qty: Some(vec![20]),
        output_qty: 0,
    });

    opcode_map.insert("74", BytecodeData {
        opcode: "PUSH21".to_string(),
        input_qty: Some(vec![21]),
        output_qty: 0,
    });

    opcode_map.insert("75", BytecodeData {
        opcode: "PUSH22".to_string(),
        input_qty: Some(vec![22]),
        output_qty: 0,
    });

    opcode_map.insert("76", BytecodeData {
        opcode: "PUSH23".to_string(),
        input_qty: Some(vec![23]),
        output_qty: 0,
    });

    opcode_map.insert("77", BytecodeData {
        opcode: "PUSH24".to_string(),
        input_qty: Some(vec![24]),
        output_qty: 0,
    });

    opcode_map.insert("78", BytecodeData {
        opcode: "PUSH25".to_string(),
        input_qty: Some(vec![25]),
        output_qty: 0,
    });

    opcode_map.insert("79", BytecodeData {
        opcode: "PUSH26".to_string(),
        input_qty: Some(vec![26]),
        output_qty: 0,
    });

    opcode_map.insert("7a", BytecodeData {
        opcode: "PUSH27".to_string(),
        input_qty: Some(vec![27]),
        output_qty: 0,
    });

    opcode_map.insert("7b", BytecodeData {
        opcode: "PUSH28".to_string(),
        input_qty: Some(vec![28]),
        output_qty: 0,
    });

    opcode_map.insert("7c", BytecodeData {
        opcode: "PUSH29".to_string(),
        input_qty: Some(vec![29]),
        output_qty: 0,
    });

    opcode_map.insert("7d", BytecodeData {
        opcode: "PUSH30".to_string(),
        input_qty: Some(vec![30]),
        output_qty: 0,
    });

    opcode_map.insert("7e", BytecodeData {
        opcode: "PUSH31".to_string(),
        input_qty: Some(vec![31]),
        output_qty: 0,
    });

    opcode_map.insert("7f", BytecodeData {
        opcode: "PUSH32".to_string(),
        input_qty: Some(vec![32]),
        output_qty: 0,
    });

    opcode_map.insert("80", BytecodeData {
        opcode: "DUP1".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("81", BytecodeData {
        opcode: "DUP2".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("82", BytecodeData {
        opcode: "DUP3".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("83", BytecodeData {
        opcode: "DUP4".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("84", BytecodeData {
        opcode: "DUP5".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("85", BytecodeData {
        opcode: "DUP6".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("86", BytecodeData {
        opcode: "DUP7".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("87", BytecodeData {
        opcode: "DUP8".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("88", BytecodeData {
        opcode: "DUP9".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("89", BytecodeData {
        opcode: "DUP10".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("8a", BytecodeData {
        opcode: "DUP11".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("8b", BytecodeData {
        opcode: "DUP12".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("8c", BytecodeData {
        opcode: "DUP13".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("8d", BytecodeData {
        opcode: "DUP14".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("8e", BytecodeData {
        opcode: "DUP15".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("8f", BytecodeData {
        opcode: "DUP16".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("90", BytecodeData {
        opcode: "SWAP1".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("91", BytecodeData {
        opcode: "SWAP2".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("92", BytecodeData {
        opcode: "SWAP3".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("93", BytecodeData {
        opcode: "SWAP4".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("94", BytecodeData {
        opcode: "SWAP5".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("95", BytecodeData {
        opcode: "SWAP6".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("96", BytecodeData {
        opcode: "SWAP7".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("97", BytecodeData {
        opcode: "SWAP8".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("98", BytecodeData {
        opcode: "SWAP9".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("99", BytecodeData {
        opcode: "SWAP10".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("9a", BytecodeData {
        opcode: "SWAP11".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("9b", BytecodeData {
        opcode: "SWAP12".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("9c", BytecodeData {
        opcode: "SWAP13".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("9d", BytecodeData {
        opcode: "SWAP14".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("9e", BytecodeData {
        opcode: "SWAP15".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("9f", BytecodeData {
        opcode: "SWAP16".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("a0", BytecodeData {
        opcode: "LOG0".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("a1", BytecodeData {
        opcode: "LOG1".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("a2", BytecodeData {
        opcode: "LOG2".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("a3", BytecodeData {
        opcode: "LOG3".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("a4", BytecodeData {
        opcode: "LOG4".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("f0", BytecodeData {
        opcode: "CREATE".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("f1", BytecodeData {
        opcode: "CALL".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("f2", BytecodeData {
        opcode: "CALLCODE".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("f3", BytecodeData {
        opcode: "RETURN".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("f4", BytecodeData {
        opcode: "DELEGATECALL".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("f5", BytecodeData {
        opcode: "CREATE2".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("fa", BytecodeData {
        opcode: "STATICCALL".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("fd", BytecodeData {
        opcode: "REVERT".to_string(),
        input_qty: None, //FIX ME - virtual stack required
        output_qty: 0,
    });

    opcode_map.insert("fe", BytecodeData {
        opcode: "INVALID".to_string(),
        input_qty: None, //FIX ME - virtual stack required
        output_qty: 0,
    });

    opcode_map.insert("ff", BytecodeData {
        opcode: "SELFDESTRUCT".to_string(),
        input_qty: None, //FIX ME - virtual stack required
        output_qty: 0,
    });

    //Questionable Invalid values Here
    opcode_map.insert("e7", BytecodeData {
        opcode: "INVALID".to_string(),
        input_qty: None, //FIX ME - virtual stack required
        output_qty: 0,
    });

    opcode_map.insert("22", BytecodeData {
        opcode: "INVALID".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("e1", BytecodeData {
        opcode: "INVALID".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("49", BytecodeData {
        opcode: "INVALID".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    opcode_map.insert("ec", BytecodeData {
        opcode: "INVALID".to_string(),
        input_qty: None, //FIX ME - up to 32 bytes can be duplicated
        output_qty: 0,
    });

    // return loaded opcode map
    opcode_map

}