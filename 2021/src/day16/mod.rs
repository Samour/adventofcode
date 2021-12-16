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

// Operator codes
const OPERATOR_SUM: i64 = 0;
const OPERATOR_PRODUCT: i64 = 1;
const OPERATOR_MIN: i64 = 2;
const OPERATOR_MAX: i64 = 3;
const OPERATOR_GT: i64 = 5;
const OPERATOR_LT: i64 = 6;
const OPERATOR_EQ: i64 = 7;

#[derive(Deserialize)]
struct Config {
  compute: String,
  packet_data: Option<String>,
  packet_file: Option<String>,
  print_expression: Option<bool>,
}

enum Operator {
  Sum,
  Product,
  Minimum,
  Maximum,
  GreaterThan,
  LessThan,
  Equal,
}

struct OperatorPacket {
  operator: Operator,
  sub_packets: Vec<Packet>,
}

impl OperatorPacket {
  fn evaluate(&self) -> Result<i64, String> {
    match self.operator {
      Operator::Sum => self.evaluate_sum(),
      Operator::Product => self.evaluate_product(),
      Operator::Minimum => self.evaluate_minimum(),
      Operator::Maximum => self.evaluate_maximum(),
      Operator::GreaterThan => self.evaluate_greater_than(),
      Operator::LessThan => self.evaluate_less_than(),
      Operator::Equal => self.evaluate_equal(),
    }
  }

  fn evaluate_sum(&self) -> Result<i64, String> {
    let mut result: i64 = 0;
    for x in &self.sub_packets {
      result += x.evaluate()?;
    }

    Ok(result)
  }

  fn evaluate_product(&self) -> Result<i64, String> {
    let mut result: i64 = 1;
    for x in &self.sub_packets {
      result *= x.evaluate()?;
    }

    Ok(result)
  }

  fn evaluate_minimum(&self) -> Result<i64, String> {
    let mut min: i64 = i64::MAX;
    for x in &self.sub_packets {
      let x_val = x.evaluate()?;
      if x_val < min {
        min = x_val;
      }
    }

    Ok(min)
  }

  fn evaluate_maximum(&self) -> Result<i64, String> {
    let mut max: i64 = 0;
    for x in &self.sub_packets {
      let x_val = x.evaluate()?;
      if x_val > max {
        max = x_val;
      }
    }

    Ok(max)
  }

  fn evaluate_greater_than(&self) -> Result<i64, String> {
    if self.sub_packets.len() != 2 {
      return Err(format!(
        "GreaterThan packet must have exactly 2 sub-packets"
      ));
    }
    if self.sub_packets[0].evaluate() > self.sub_packets[1].evaluate() {
      Ok(1)
    } else {
      Ok(0)
    }
  }

  fn evaluate_less_than(&self) -> Result<i64, String> {
    if self.sub_packets.len() != 2 {
      return Err(format!("LessThan packet must have exactly 2 sub-packets"));
    }
    if self.sub_packets[0].evaluate() < self.sub_packets[1].evaluate() {
      Ok(1)
    } else {
      Ok(0)
    }
  }

  fn evaluate_equal(&self) -> Result<i64, String> {
    if self.sub_packets.len() != 2 {
      return Err(format!("Equal packet must have exactly 2 sub-packets"));
    }
    if self.sub_packets[0].evaluate() == self.sub_packets[1].evaluate() {
      Ok(1)
    } else {
      Ok(0)
    }
  }

  fn render(&self) -> String {
    let name = match self.operator {
      Operator::Sum => "Sum",
      Operator::Product => "Product",
      Operator::Minimum => "Minimum",
      Operator::Maximum => "Maximum",
      Operator::GreaterThan => "GreaterThan",
      Operator::LessThan => "LessThan",
      Operator::Equal => "Equal",
    };
    let contents: String = self
      .sub_packets
      .iter()
      .map(|p| p.render())
      .collect::<Vec<String>>()
      .join(" , ");
    format!("{}( {} )", name, contents)
  }
}

enum PacketContents {
  IntValue(i64),
  Operator(OperatorPacket),
}

struct Packet {
  version_number: i64,
  type_code: i64,
  contents: PacketContents,
}

impl Packet {
  fn evaluate(&self) -> Result<i64, String> {
    match &self.contents {
      PacketContents::IntValue(i) => Ok(*i),
      PacketContents::Operator(contents) => contents.evaluate(),
    }
  }

  fn render(&self) -> String {
    match &self.contents {
      PacketContents::IntValue(i) => format!("{}", i),
      PacketContents::Operator(contents) => contents.render(),
    }
  }
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

  fn parse_operator_type(&self, type_code: i64) -> Result<Operator, String> {
    match type_code {
      OPERATOR_SUM => Ok(Operator::Sum),
      OPERATOR_PRODUCT => Ok(Operator::Product),
      OPERATOR_MIN => Ok(Operator::Minimum),
      OPERATOR_MAX => Ok(Operator::Maximum),
      OPERATOR_GT => Ok(Operator::GreaterThan),
      OPERATOR_LT => Ok(Operator::LessThan),
      OPERATOR_EQ => Ok(Operator::Equal),
      _ => Err(format!("Unrecognized operator code")),
    }
  }

  fn parse_operator_contents(&mut self, type_code: i64) -> Result<PacketContents, String> {
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

    Ok(PacketContents::Operator(OperatorPacket {
      operator: self.parse_operator_type(type_code)?,
      sub_packets,
    }))
  }

  fn parse(&mut self) -> Result<Packet, String> {
    let version_number = self.serve_int(LEN_VERSION_NO)?;
    let type_code = self.serve_int(LEN_TYPE_NO)?;
    let contents = if type_code == TYPE_CODE_VALUE {
      self.parse_int_value_contents()?
    } else {
      self.parse_operator_contents(type_code)?
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
    PacketContents::Operator(operator_packet) => {
      for p in &operator_packet.sub_packets {
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
  if context.config.print_expression.unwrap_or(false) {
    writer.write(|| root_packet.render());
  }
  match context.config.compute.as_str() {
    "versions_sum" => {
      let result = sum_packet_versions(&root_packet);
      writer.write(|| format!("Sum of all packet versions: {}", result));
      Ok(format!("{}", result))
    }
    "evaluation" => {
      let result = root_packet.evaluate()?;
      writer.write(|| format!("Evaluation of packet comes to: {}", result));
      Ok(format!("{}", result))
    }
    _ => Err(format!("Unrecognized compute option")),
  }
}
