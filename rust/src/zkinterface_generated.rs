// automatically generated by the FlatBuffers compiler, do not modify


pub mod zkinterface {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

/// The messages that the caller and gadget can exchange.
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Message {
  NONE = 0,
  Circuit = 1,
  R1CSConstraints = 2,
  Witness = 3,

}

const ENUM_MIN_MESSAGE: u8 = 0;
const ENUM_MAX_MESSAGE: u8 = 3;

impl<'a> flatbuffers::Follow<'a> for Message {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for Message {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const Message;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const Message;
    unsafe { *p }
  }
}

impl flatbuffers::Push for Message {
    type Output = Message;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<Message>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_MESSAGE:[Message; 4] = [
  Message::NONE,
  Message::Circuit,
  Message::R1CSConstraints,
  Message::Witness
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_MESSAGE:[&'static str; 4] = [
    "NONE",
    "Circuit",
    "R1CSConstraints",
    "Witness"
];

pub fn enum_name_message(e: Message) -> &'static str {
  let index: usize = e as usize;
  ENUM_NAMES_MESSAGE[index]
}

pub struct MessageUnionTableOffset {}
/// A description of the connection of a circuit or sub-circuit.
/// This can be a complete circuit ready for proving,
/// or a part of a circuit being built.
pub enum CircuitOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Circuit<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Circuit<'a> {
    type Inner = Circuit<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Circuit<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Circuit {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args CircuitArgs<'args>) -> flatbuffers::WIPOffset<Circuit<'bldr>> {
      let mut builder = CircuitBuilder::new(_fbb);
      builder.add_free_variable_id(args.free_variable_id);
      if let Some(x) = args.configuration { builder.add_configuration(x); }
      if let Some(x) = args.field_order { builder.add_field_order(x); }
      if let Some(x) = args.connections { builder.add_connections(x); }
      builder.add_witness_generation(args.witness_generation);
      builder.add_r1cs_generation(args.r1cs_generation);
      builder.finish()
    }

    pub const VT_CONNECTIONS: flatbuffers::VOffsetT = 4;
    pub const VT_FREE_VARIABLE_ID: flatbuffers::VOffsetT = 6;
    pub const VT_R1CS_GENERATION: flatbuffers::VOffsetT = 8;
    pub const VT_WITNESS_GENERATION: flatbuffers::VOffsetT = 10;
    pub const VT_FIELD_ORDER: flatbuffers::VOffsetT = 12;
    pub const VT_CONFIGURATION: flatbuffers::VOffsetT = 14;

  /// Variables to use as connection to the sub-circuit.
  /// - Variables to use as input connections to the gadget.
  /// - Or variables to use as output connections from the gadget.
  /// - Variables are allocated by the sender of this message.
  /// - The same structure must be provided for R1CS and witness generation.
  #[inline]
  pub fn connections(&self) -> Option<VariableValues<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<VariableValues<'a>>>(Circuit::VT_CONNECTIONS, None)
  }
  /// First variable ID free after this connection.
  /// A variable ID greater than all IDs allocated by the sender of this message.
  /// The recipient of this message can allocate new IDs greater than `free_variable_id`.
  #[inline]
  pub fn free_variable_id(&self) -> u64 {
    self._tab.get::<u64>(Circuit::VT_FREE_VARIABLE_ID, Some(0)).unwrap()
  }
  /// Whether constraints should be generated.
  #[inline]
  pub fn r1cs_generation(&self) -> bool {
    self._tab.get::<bool>(Circuit::VT_R1CS_GENERATION, Some(false)).unwrap()
  }
  /// Whether an witness should be generated.
  /// Provide witness values to the gadget.
  #[inline]
  pub fn witness_generation(&self) -> bool {
    self._tab.get::<bool>(Circuit::VT_WITNESS_GENERATION, Some(false)).unwrap()
  }
  /// The order of the field used by the current system.
  /// A BigInt.
  #[inline]
  pub fn field_order(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Circuit::VT_FIELD_ORDER, None).map(|v| v.safe_slice())
  }
  /// Optional: Any static parameter that may influence the instance
  /// construction. Parameters can be standard, conventional, or custom.
  /// Example: function_name, if a gadget supports multiple function variants.
  /// Example: the depth of a Merkle tree.
  /// Counter-example: a Merkle path is not configuration (rather witness).
  #[inline]
  pub fn configuration(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<KeyValue<'a>>>>>(Circuit::VT_CONFIGURATION, None)
  }
}

pub struct CircuitArgs<'a> {
    pub connections: Option<flatbuffers::WIPOffset<VariableValues<'a >>>,
    pub free_variable_id: u64,
    pub r1cs_generation: bool,
    pub witness_generation: bool,
    pub field_order: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
    pub configuration: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<KeyValue<'a >>>>>,
}
impl<'a> Default for CircuitArgs<'a> {
    #[inline]
    fn default() -> Self {
        CircuitArgs {
            connections: None,
            free_variable_id: 0,
            r1cs_generation: false,
            witness_generation: false,
            field_order: None,
            configuration: None,
        }
    }
}
pub struct CircuitBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> CircuitBuilder<'a, 'b> {
  #[inline]
  pub fn add_connections(&mut self, connections: flatbuffers::WIPOffset<VariableValues<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<VariableValues>>(Circuit::VT_CONNECTIONS, connections);
  }
  #[inline]
  pub fn add_free_variable_id(&mut self, free_variable_id: u64) {
    self.fbb_.push_slot::<u64>(Circuit::VT_FREE_VARIABLE_ID, free_variable_id, 0);
  }
  #[inline]
  pub fn add_r1cs_generation(&mut self, r1cs_generation: bool) {
    self.fbb_.push_slot::<bool>(Circuit::VT_R1CS_GENERATION, r1cs_generation, false);
  }
  #[inline]
  pub fn add_witness_generation(&mut self, witness_generation: bool) {
    self.fbb_.push_slot::<bool>(Circuit::VT_WITNESS_GENERATION, witness_generation, false);
  }
  #[inline]
  pub fn add_field_order(&mut self, field_order: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Circuit::VT_FIELD_ORDER, field_order);
  }
  #[inline]
  pub fn add_configuration(&mut self, configuration: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<KeyValue<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Circuit::VT_CONFIGURATION, configuration);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> CircuitBuilder<'a, 'b> {
    let start = _fbb.start_table();
    CircuitBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Circuit<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

/// Generic key-value for custom attributes.
pub enum KeyValueOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct KeyValue<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for KeyValue<'a> {
    type Inner = KeyValue<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> KeyValue<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        KeyValue {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args KeyValueArgs<'args>) -> flatbuffers::WIPOffset<KeyValue<'bldr>> {
      let mut builder = KeyValueBuilder::new(_fbb);
      if let Some(x) = args.value { builder.add_value(x); }
      if let Some(x) = args.key { builder.add_key(x); }
      builder.finish()
    }

    pub const VT_KEY: flatbuffers::VOffsetT = 4;
    pub const VT_VALUE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn key(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(KeyValue::VT_KEY, None)
  }
  #[inline]
  pub fn value(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(KeyValue::VT_VALUE, None).map(|v| v.safe_slice())
  }
}

pub struct KeyValueArgs<'a> {
    pub key: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub value: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
}
impl<'a> Default for KeyValueArgs<'a> {
    #[inline]
    fn default() -> Self {
        KeyValueArgs {
            key: None,
            value: None,
        }
    }
}
pub struct KeyValueBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> KeyValueBuilder<'a, 'b> {
  #[inline]
  pub fn add_key(&mut self, key: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(KeyValue::VT_KEY, key);
  }
  #[inline]
  pub fn add_value(&mut self, value: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(KeyValue::VT_VALUE, value);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> KeyValueBuilder<'a, 'b> {
    let start = _fbb.start_table();
    KeyValueBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<KeyValue<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

/// Report constraints to be added to the constraints system.
/// To send to the stream of constraints.
pub enum R1CSConstraintsOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct R1CSConstraints<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for R1CSConstraints<'a> {
    type Inner = R1CSConstraints<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> R1CSConstraints<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        R1CSConstraints {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args R1CSConstraintsArgs<'args>) -> flatbuffers::WIPOffset<R1CSConstraints<'bldr>> {
      let mut builder = R1CSConstraintsBuilder::new(_fbb);
      if let Some(x) = args.constraints { builder.add_constraints(x); }
      builder.finish()
    }

    pub const VT_CONSTRAINTS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn constraints(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BilinearConstraint<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<BilinearConstraint<'a>>>>>(R1CSConstraints::VT_CONSTRAINTS, None)
  }
}

pub struct R1CSConstraintsArgs<'a> {
    pub constraints: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<BilinearConstraint<'a >>>>>,
}
impl<'a> Default for R1CSConstraintsArgs<'a> {
    #[inline]
    fn default() -> Self {
        R1CSConstraintsArgs {
            constraints: None,
        }
    }
}
pub struct R1CSConstraintsBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> R1CSConstraintsBuilder<'a, 'b> {
  #[inline]
  pub fn add_constraints(&mut self, constraints: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<BilinearConstraint<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(R1CSConstraints::VT_CONSTRAINTS, constraints);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> R1CSConstraintsBuilder<'a, 'b> {
    let start = _fbb.start_table();
    R1CSConstraintsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<R1CSConstraints<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

/// An R1CS constraint between variables.
pub enum BilinearConstraintOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct BilinearConstraint<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for BilinearConstraint<'a> {
    type Inner = BilinearConstraint<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> BilinearConstraint<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        BilinearConstraint {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args BilinearConstraintArgs<'args>) -> flatbuffers::WIPOffset<BilinearConstraint<'bldr>> {
      let mut builder = BilinearConstraintBuilder::new(_fbb);
      if let Some(x) = args.linear_combination_c { builder.add_linear_combination_c(x); }
      if let Some(x) = args.linear_combination_b { builder.add_linear_combination_b(x); }
      if let Some(x) = args.linear_combination_a { builder.add_linear_combination_a(x); }
      builder.finish()
    }

    pub const VT_LINEAR_COMBINATION_A: flatbuffers::VOffsetT = 4;
    pub const VT_LINEAR_COMBINATION_B: flatbuffers::VOffsetT = 6;
    pub const VT_LINEAR_COMBINATION_C: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn linear_combination_a(&self) -> Option<VariableValues<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<VariableValues<'a>>>(BilinearConstraint::VT_LINEAR_COMBINATION_A, None)
  }
  #[inline]
  pub fn linear_combination_b(&self) -> Option<VariableValues<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<VariableValues<'a>>>(BilinearConstraint::VT_LINEAR_COMBINATION_B, None)
  }
  #[inline]
  pub fn linear_combination_c(&self) -> Option<VariableValues<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<VariableValues<'a>>>(BilinearConstraint::VT_LINEAR_COMBINATION_C, None)
  }
}

pub struct BilinearConstraintArgs<'a> {
    pub linear_combination_a: Option<flatbuffers::WIPOffset<VariableValues<'a >>>,
    pub linear_combination_b: Option<flatbuffers::WIPOffset<VariableValues<'a >>>,
    pub linear_combination_c: Option<flatbuffers::WIPOffset<VariableValues<'a >>>,
}
impl<'a> Default for BilinearConstraintArgs<'a> {
    #[inline]
    fn default() -> Self {
        BilinearConstraintArgs {
            linear_combination_a: None,
            linear_combination_b: None,
            linear_combination_c: None,
        }
    }
}
pub struct BilinearConstraintBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> BilinearConstraintBuilder<'a, 'b> {
  #[inline]
  pub fn add_linear_combination_a(&mut self, linear_combination_a: flatbuffers::WIPOffset<VariableValues<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<VariableValues>>(BilinearConstraint::VT_LINEAR_COMBINATION_A, linear_combination_a);
  }
  #[inline]
  pub fn add_linear_combination_b(&mut self, linear_combination_b: flatbuffers::WIPOffset<VariableValues<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<VariableValues>>(BilinearConstraint::VT_LINEAR_COMBINATION_B, linear_combination_b);
  }
  #[inline]
  pub fn add_linear_combination_c(&mut self, linear_combination_c: flatbuffers::WIPOffset<VariableValues<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<VariableValues>>(BilinearConstraint::VT_LINEAR_COMBINATION_C, linear_combination_c);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> BilinearConstraintBuilder<'a, 'b> {
    let start = _fbb.start_table();
    BilinearConstraintBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<BilinearConstraint<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

/// Report local assignments computed by the gadget.
/// To send to the stream of assigned variables.
/// Does not include input and output variables.
pub enum WitnessOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Witness<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Witness<'a> {
    type Inner = Witness<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Witness<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Witness {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args WitnessArgs<'args>) -> flatbuffers::WIPOffset<Witness<'bldr>> {
      let mut builder = WitnessBuilder::new(_fbb);
      if let Some(x) = args.values { builder.add_values(x); }
      builder.finish()
    }

    pub const VT_VALUES: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn values(&self) -> Option<VariableValues<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<VariableValues<'a>>>(Witness::VT_VALUES, None)
  }
}

pub struct WitnessArgs<'a> {
    pub values: Option<flatbuffers::WIPOffset<VariableValues<'a >>>,
}
impl<'a> Default for WitnessArgs<'a> {
    #[inline]
    fn default() -> Self {
        WitnessArgs {
            values: None,
        }
    }
}
pub struct WitnessBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> WitnessBuilder<'a, 'b> {
  #[inline]
  pub fn add_values(&mut self, values: flatbuffers::WIPOffset<VariableValues<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<VariableValues>>(Witness::VT_VALUES, values);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> WitnessBuilder<'a, 'b> {
    let start = _fbb.start_table();
    WitnessBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Witness<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

/// Concrete variable values.
/// Used for linear combinations and assignments.
pub enum VariableValuesOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct VariableValues<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for VariableValues<'a> {
    type Inner = VariableValues<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> VariableValues<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        VariableValues {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args VariableValuesArgs<'args>) -> flatbuffers::WIPOffset<VariableValues<'bldr>> {
      let mut builder = VariableValuesBuilder::new(_fbb);
      if let Some(x) = args.info { builder.add_info(x); }
      if let Some(x) = args.values { builder.add_values(x); }
      if let Some(x) = args.variable_ids { builder.add_variable_ids(x); }
      builder.finish()
    }

    pub const VT_VARIABLE_IDS: flatbuffers::VOffsetT = 4;
    pub const VT_VALUES: flatbuffers::VOffsetT = 6;
    pub const VT_INFO: flatbuffers::VOffsetT = 8;

  /// The IDs of the variables being assigned to.
  #[inline]
  pub fn variable_ids(&self) -> Option<flatbuffers::Vector<'a, u64>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u64>>>(VariableValues::VT_VARIABLE_IDS, None)
  }
  /// Optional: Field elements assigned to variables.
  /// Contiguous BigInts in the same order as variable_ids.
  ///
  /// The field in use is defined in `instance.field_order`.
  ///
  /// The size of an element representation is determined by:
  ///     element size = elements.length / variable_ids.length
  ///
  /// The element representation may be truncated and therefore shorter
  /// than the canonical representation. Truncated bytes are treated as zeros.
  #[inline]
  pub fn values(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(VariableValues::VT_VALUES, None).map(|v| v.safe_slice())
  }
  /// Optional: Any complementary info that may be useful to the recipient.
  /// Example: a Merkle authentication path.
  #[inline]
  pub fn info(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<KeyValue<'a>>>>>(VariableValues::VT_INFO, None)
  }
}

pub struct VariableValuesArgs<'a> {
    pub variable_ids: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u64>>>,
    pub values: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
    pub info: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<KeyValue<'a >>>>>,
}
impl<'a> Default for VariableValuesArgs<'a> {
    #[inline]
    fn default() -> Self {
        VariableValuesArgs {
            variable_ids: None,
            values: None,
            info: None,
        }
    }
}
pub struct VariableValuesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> VariableValuesBuilder<'a, 'b> {
  #[inline]
  pub fn add_variable_ids(&mut self, variable_ids: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u64>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(VariableValues::VT_VARIABLE_IDS, variable_ids);
  }
  #[inline]
  pub fn add_values(&mut self, values: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(VariableValues::VT_VALUES, values);
  }
  #[inline]
  pub fn add_info(&mut self, info: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<KeyValue<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(VariableValues::VT_INFO, info);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> VariableValuesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    VariableValuesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<VariableValues<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum RootOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Root<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Root<'a> {
    type Inner = Root<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Root<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Root {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args RootArgs) -> flatbuffers::WIPOffset<Root<'bldr>> {
      let mut builder = RootBuilder::new(_fbb);
      if let Some(x) = args.message { builder.add_message(x); }
      builder.add_message_type(args.message_type);
      builder.finish()
    }

    pub const VT_MESSAGE_TYPE: flatbuffers::VOffsetT = 4;
    pub const VT_MESSAGE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn message_type(&self) -> Message {
    self._tab.get::<Message>(Root::VT_MESSAGE_TYPE, Some(Message::NONE)).unwrap()
  }
  #[inline]
  pub fn message(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Root::VT_MESSAGE, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_circuit(&self) -> Option<Circuit<'a>> {
    if self.message_type() == Message::Circuit {
      self.message().map(|u| Circuit::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_r1csconstraints(&self) -> Option<R1CSConstraints<'a>> {
    if self.message_type() == Message::R1CSConstraints {
      self.message().map(|u| R1CSConstraints::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_witness(&self) -> Option<Witness<'a>> {
    if self.message_type() == Message::Witness {
      self.message().map(|u| Witness::init_from_table(u))
    } else {
      None
    }
  }

}

pub struct RootArgs {
    pub message_type: Message,
    pub message: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for RootArgs {
    #[inline]
    fn default() -> Self {
        RootArgs {
            message_type: Message::NONE,
            message: None,
        }
    }
}
pub struct RootBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RootBuilder<'a, 'b> {
  #[inline]
  pub fn add_message_type(&mut self, message_type: Message) {
    self.fbb_.push_slot::<Message>(Root::VT_MESSAGE_TYPE, message_type, Message::NONE);
  }
  #[inline]
  pub fn add_message(&mut self, message: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Root::VT_MESSAGE, message);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RootBuilder<'a, 'b> {
    let start = _fbb.start_table();
    RootBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Root<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_root<'a>(buf: &'a [u8]) -> Root<'a> {
  flatbuffers::get_root::<Root<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_root<'a>(buf: &'a [u8]) -> Root<'a> {
  flatbuffers::get_size_prefixed_root::<Root<'a>>(buf)
}

pub const ROOT_IDENTIFIER: &'static str = "zkif";

#[inline]
pub fn root_buffer_has_identifier(buf: &[u8]) -> bool {
  return flatbuffers::buffer_has_identifier(buf, ROOT_IDENTIFIER, false);
}

#[inline]
pub fn root_size_prefixed_buffer_has_identifier(buf: &[u8]) -> bool {
  return flatbuffers::buffer_has_identifier(buf, ROOT_IDENTIFIER, true);
}

pub const ROOT_EXTENSION: &'static str = "zkif";

#[inline]
pub fn finish_root_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Root<'a>>) {
  fbb.finish(root, Some(ROOT_IDENTIFIER));
}

#[inline]
pub fn finish_size_prefixed_root_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Root<'a>>) {
  fbb.finish_size_prefixed(root, Some(ROOT_IDENTIFIER));
}
}  // pub mod zkinterface

