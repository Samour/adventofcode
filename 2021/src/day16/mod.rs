use serde::Deserialize;
use std::collections::LinkedList;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

// Lengths of fixed-length fields
const LEN_VERSION_NO: usize = 3;
const LEN_TYPE_NO: usize = 3;
const LEN_VALUE_PART: usize = 5;
const LEN_LENGTH_INDICATOR: usize = 1;
const LEN_BITS_LENGTH: usize = 15;
const LEN_PACKETS_LENGTH: usize = 11;

// Metadata values
const TYPE_CODE_VALUE: i64 = 4;
const LENGTH_INDICATOR_BITS: i64 = 0;

// Values used in int parsing
const VALUE_MASK: i64 = 0x0f;
const VALUE_CONTINUE_MASK: i64 = 0x10;
const VALUE_BLOCK_MAG: i64 = 16;

#[derive(Deserialize)]
struct Config {
  packet_data: Option<String>,
  packet_file: Option<String>,
}

enum PacketContents {
  IntValue(i64),
  Operator(Vec<Packet>),
}

struct Packet {
  version_number: i64,
  type_code: i64,
  contents: PacketContents,
}

struct PacketParser {
  remaining_bits: usize,
  source: LinkedList<usize>,
}

impl PacketParser {
  fn new(source: LinkedList<usize>) -> PacketParser {
    let remaining_bits = source.len();
    PacketParser {
      remaining_bits,
      source,
    }
  }

  fn serve_int(&mut self, len: usize) -> Result<i64, String> {
    let mut result: i64 = 0;
    for _ in 0..len {
      result *= 2;
      result += self
        .source
        .pop_front()
        .ok_or_else(|| format!("Attempted to pop next bit, but none left"))? as i64;
    }
    self.remaining_bits -= len;

    Ok(result)
  }

  fn parse_int_value_contents(&mut self) -> Result<PacketContents, String> {
    let mut value: i64 = 0;
    loop {
      value *= VALUE_BLOCK_MAG;
      let next_block = self.serve_int(LEN_VALUE_PART)?;
      value += next_block & VALUE_MASK;
      if next_block & VALUE_CONTINUE_MASK == 0 {
        break;
      }
    }

    Ok(PacketContents::IntValue(value))
  }

  fn parse_operator_contents(&mut self) -> Result<PacketContents, String> {
    let length_indicator = self.serve_int(LEN_LENGTH_INDICATOR)?;
    let mut sub_packets: Vec<Packet> = Vec::new();
    if length_indicator == LENGTH_INDICATOR_BITS {
      let sub_packets_length = self.serve_int(LEN_BITS_LENGTH)?;
      let stop_reading_at = self.remaining_bits as i64 - sub_packets_length;
      while self.remaining_bits as i64 > stop_reading_at {
        sub_packets.push(self.parse()?);
      }
    } else {
      let sub_packets_count = self.serve_int(LEN_PACKETS_LENGTH)?;
      while (sub_packets.len() as i64) < sub_packets_count {
        sub_packets.push(self.parse()?);
      }
    }

    Ok(PacketContents::Operator(sub_packets))
  }

  fn parse(&mut self) -> Result<Packet, String> {
    let version_number = self.serve_int(LEN_VERSION_NO)?;
    let type_code = self.serve_int(LEN_TYPE_NO)?;
    let contents = if type_code == TYPE_CODE_VALUE {
      self.parse_int_value_contents()?
    } else {
      self.parse_operator_contents()?
    };

    Ok(Packet {
      version_number,
      type_code,
      contents,
    })
  }
}

fn sum_packet_versions(packet: &Packet) -> i64 {
  let mut result = packet.version_number;
  match &packet.contents {
    PacketContents::Operator(sub_packets) => {
      for p in sub_packets {
        result += sum_packet_versions(p);
      }
    }
    _ => {}
  }

  result
}

fn read_hex_packet(context: &Context<Config>) -> Result<String, String> {
  match &context.config.packet_data {
    Some(p) => return Ok(p.clone()),
    None => {}
  }
  let fname = context.config.packet_file.as_ref().ok_or(format!(
    "One of packet_data or packet_file must be specified"
  ))?;
  let mut hex_packet = String::new();
  for line in context.load_data(fname)?.split("\n") {
    hex_packet.push_str(line);
  }

  Ok(hex_packet)
}

fn hex_to_bits(hex_repr: String) -> Result<LinkedList<usize>, String> {
  let mut result: LinkedList<usize> = LinkedList::new();
  for c in hex_repr.chars() {
    let mut value: usize = match c {
      '0' => 0x00,
      '1' => 0x01,
      '2' => 0x02,
      '3' => 0x03,
      '4' => 0x04,
      '5' => 0x05,
      '6' => 0x06,
      '7' => 0x07,
      '8' => 0x08,
      '9' => 0x09,
      'A' => 0x0a,
      'B' => 0x0b,
      'C' => 0x0c,
      'D' => 0x0d,
      'E' => 0x0e,
      'F' => 0x0f,
      _ => return Err(format!("Unrecognized symbol in hex string")),
    };
    for i in 0..4 {
      if value & (0x01 << (3 - i)) > 0 {
        result.push_back(1);
      } else {
        result.push_back(0);
      }
    }
  }

  Ok(result)
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<String, String> {
  let context: Context<Config> = factory.create()?;
  let hex_raw = read_hex_packet(&context)?;
  let bits = hex_to_bits(hex_raw)?;
  let root_packet = PacketParser::new(bits).parse()?;
  let result = sum_packet_versions(&root_packet);
  writer.write(|| format!("Sum of all packet versions: {}", result));
  Ok(format!("{}", result))
}
