use elf::endian::{BigEndian, LittleEndian};
use elf::ElfBytes;
use serde::{Deserialize, Serialize};
use solana_account_decoder::{UiAccount, UiAccountData, UiAccountEncoding};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct RpcKeyedAccount {
    pubkey: String,
    account: UiAccount,
}

pub const RUNTIME_UNALTERED: &str = "RUNTIME";
pub const RUNTIME_SIMULATED: &str = "v1     ";
pub const DIR: &str = "../client/mocks";

fn open_program_file(path: &str) -> File {
    let full_path = Path::new(DIR).clone().join(Path::new(path));
    File::open(full_path.to_str().unwrap()).unwrap()
}

fn save_program_data(program_data: &RpcKeyedAccount, path: &str) {
    let full_path = Path::new(DIR).clone().join(Path::new(path));
    let mut file = File::create(full_path).unwrap();
    let str = serde_json::to_string(program_data).unwrap();
    file.write(str.as_bytes()).unwrap();
}

/// Rewrites the program's binary info with modified
/// ELF data
fn rewrite_program_info(
    pubkey: &String,
    copied_account: UiAccount,
    program_data: &Vec<u8>,
) -> RpcKeyedAccount {
    RpcKeyedAccount {
        pubkey: pubkey.clone(),
        account: UiAccount {
            data: UiAccountData::Binary(base64::encode(&program_data), UiAccountEncoding::Base64),
            ..copied_account
        },
    }
}

fn modify_program_data() {
    let mut file = open_program_file("program.info");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let data: RpcKeyedAccount = serde_json::from_str(&data).unwrap();
    let account = data.account;

    let copied_account = account.clone();
    match account.data {
        UiAccountData::Json(_) => {
            panic!("Program data is not binary");
        }
        UiAccountData::LegacyBinary(_blob) => {
            panic!("Program data is not binary");
        }
        UiAccountData::Binary(serialized_data, encoding) => match encoding {
            UiAccountEncoding::Base58 => {
                panic!("Program data is not binary");
            }
            UiAccountEncoding::Base64 => {
                let mut program_data = base64::decode(serialized_data).unwrap();
                handle_program_data(&mut program_data);

                let modified_data =
                    rewrite_program_info(&data.pubkey, copied_account, &program_data);
                save_program_data(&modified_data, "program-sim.info")
            }
            UiAccountEncoding::Base64Zstd => {
                panic!("Program data is not binary");
            }
            UiAccountEncoding::Binary | UiAccountEncoding::JsonParsed => {
                panic!("Program data is not binary");
            }
        },
    }
}

fn run_test_validator() {
    let command = Command::new("./xray")
        .arg("-r")
        .arg("--account")
        .arg("9Y85P3QZ85UrgqKBammFMQh5anS2zENXTawJ61Hq9jGR")
        .arg("../client/mocks/program-sim.info")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute process");
    let output = command.wait_with_output().expect("failed to wait on child");
    println!("{:?}", output);
}

fn handle_program_data(bytes: &mut Vec<u8>) {
    let file = ElfBytes::<LittleEndian>::minimal_parse(&bytes).expect("Open program ELF file");

    let runtime_header = file
        .section_header_by_name(".runtime.txt")
        .expect("shdrs should parse")
        .expect("Program ELF file should have a .runtime.txt section header");

    let runtime_data = file
        .section_data(&runtime_header)
        .expect("Program should have .runtime.txt section data");
    println!("Runtime data: {:?}", runtime_data);

    let (pointer_bytes, len_bytes) = runtime_data.0.split_at(8);

    // HACK: Taking the first word of a u64 pointer
    let pointer: u32 = u32::from_le_bytes(pointer_bytes[4..].try_into().unwrap());
    println!("Pointer: {}", format!("{:x}", pointer));

    let len = u64::from_le_bytes(len_bytes.try_into().unwrap());

    let string_bytes = bytes
        .get(pointer as usize..(pointer as u64 + len) as usize)
        .unwrap();
    let runtime_str = std::str::from_utf8(string_bytes).unwrap();
    println!("Runtime String: {:?}", runtime_str);
    assert_eq!(runtime_str, RUNTIME_UNALTERED);

    bytes[pointer as usize..(pointer as u64 + len) as usize]
        .copy_from_slice(RUNTIME_SIMULATED.as_bytes());
}

fn main() {
    modify_program_data();
    run_test_validator();
}
