// Generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: test/test.capnp


#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum TestEnum {
  Foo = 0,
  Bar = 1,
  Baz = 2,
  Qux = 3,
  Quux = 4,
  Corge = 5,
  Grault = 6,
  Garply = 7,
}
impl ::capnp::traits::FromU16 for TestEnum {
  #[inline]
  fn from_u16(value: u16) -> ::std::result::Result<TestEnum, ::capnp::NotInSchema> {
    match value {
      0 => ::std::result::Result::Ok(TestEnum::Foo),
      1 => ::std::result::Result::Ok(TestEnum::Bar),
      2 => ::std::result::Result::Ok(TestEnum::Baz),
      3 => ::std::result::Result::Ok(TestEnum::Qux),
      4 => ::std::result::Result::Ok(TestEnum::Quux),
      5 => ::std::result::Result::Ok(TestEnum::Corge),
      6 => ::std::result::Result::Ok(TestEnum::Grault),
      7 => ::std::result::Result::Ok(TestEnum::Garply),
      n => ::std::result::Result::Err(::capnp::NotInSchema(n)),
    }
  }
}
impl ::capnp::traits::ToU16 for TestEnum {
  #[inline]
  fn to_u16(self) -> u16 { self as u16 }
}
impl ::capnp::traits::HasTypeId for TestEnum {
  #[inline]
  fn type_id() -> u64 { 0xcf094e49c0fd52ebu64 }
}

pub mod test_all_types {
  #![allow(unused_imports)]
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{text, data, Result};
  use capnp::private::layout;
  use capnp::traits::{FromStructBuilder, FromStructReader};
  use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};

  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader: layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader: reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>) -> Result<Reader<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(try!(reader.get_struct(::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn borrow<'b>(&'b self) -> Reader<'b,> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_void_field(self) -> () {
      ()
    }
    #[inline]
    pub fn get_bool_field(self) -> bool {
      self.reader.get_bool_field(0)
    }
    #[inline]
    pub fn get_int8_field(self) -> i8 {
      self.reader.get_data_field::<i8>(1)
    }
    #[inline]
    pub fn get_int16_field(self) -> i16 {
      self.reader.get_data_field::<i16>(1)
    }
    #[inline]
    pub fn get_int32_field(self) -> i32 {
      self.reader.get_data_field::<i32>(1)
    }
    #[inline]
    pub fn get_int64_field(self) -> i64 {
      self.reader.get_data_field::<i64>(1)
    }
    #[inline]
    pub fn get_u_int8_field(self) -> u8 {
      self.reader.get_data_field::<u8>(16)
    }
    #[inline]
    pub fn get_u_int16_field(self) -> u16 {
      self.reader.get_data_field::<u16>(9)
    }
    #[inline]
    pub fn get_u_int32_field(self) -> u32 {
      self.reader.get_data_field::<u32>(5)
    }
    #[inline]
    pub fn get_u_int64_field(self) -> u64 {
      self.reader.get_data_field::<u64>(3)
    }
    #[inline]
    pub fn get_float32_field(self) -> f32 {
      self.reader.get_data_field::<f32>(8)
    }
    #[inline]
    pub fn get_float64_field(self) -> f64 {
      self.reader.get_data_field::<f64>(5)
    }
    #[inline]
    pub fn get_text_field(self) -> Result<text::Reader<'a>> {
      self.reader.get_pointer_field(0).get_text(::std::ptr::null(), 0)
    }
    pub fn has_text_field(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_data_field(self) -> Result<data::Reader<'a>> {
      self.reader.get_pointer_field(1).get_data(::std::ptr::null(), 0)
    }
    pub fn has_data_field(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_struct_field(self) -> Result<::test_capnp::test_all_types::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(2))
    }
    pub fn has_struct_field(&self) -> bool {
      !self.reader.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_enum_field(self) -> ::std::result::Result<::test_capnp::TestEnum,::capnp::NotInSchema> {
      ::capnp::traits::FromU16::from_u16(self.reader.get_data_field::<u16>(18))
    }
    #[inline]
    pub fn get_interface_field(self) -> () {
      ()
    }
    #[inline]
    pub fn get_void_list(self) -> Result<primitive_list::Reader<'a,()>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(3))
    }
    pub fn has_void_list(&self) -> bool {
      !self.reader.get_pointer_field(3).is_null()
    }
    #[inline]
    pub fn get_bool_list(self) -> Result<primitive_list::Reader<'a,bool>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(4))
    }
    pub fn has_bool_list(&self) -> bool {
      !self.reader.get_pointer_field(4).is_null()
    }
    #[inline]
    pub fn get_int8_list(self) -> Result<primitive_list::Reader<'a,i8>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(5))
    }
    pub fn has_int8_list(&self) -> bool {
      !self.reader.get_pointer_field(5).is_null()
    }
    #[inline]
    pub fn get_int16_list(self) -> Result<primitive_list::Reader<'a,i16>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(6))
    }
    pub fn has_int16_list(&self) -> bool {
      !self.reader.get_pointer_field(6).is_null()
    }
    #[inline]
    pub fn get_int32_list(self) -> Result<primitive_list::Reader<'a,i32>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(7))
    }
    pub fn has_int32_list(&self) -> bool {
      !self.reader.get_pointer_field(7).is_null()
    }
    #[inline]
    pub fn get_int64_list(self) -> Result<primitive_list::Reader<'a,i64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(8))
    }
    pub fn has_int64_list(&self) -> bool {
      !self.reader.get_pointer_field(8).is_null()
    }
    #[inline]
    pub fn get_u_int8_list(self) -> Result<primitive_list::Reader<'a,u8>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(9))
    }
    pub fn has_u_int8_list(&self) -> bool {
      !self.reader.get_pointer_field(9).is_null()
    }
    #[inline]
    pub fn get_u_int16_list(self) -> Result<primitive_list::Reader<'a,u16>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(10))
    }
    pub fn has_u_int16_list(&self) -> bool {
      !self.reader.get_pointer_field(10).is_null()
    }
    #[inline]
    pub fn get_u_int32_list(self) -> Result<primitive_list::Reader<'a,u32>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(11))
    }
    pub fn has_u_int32_list(&self) -> bool {
      !self.reader.get_pointer_field(11).is_null()
    }
    #[inline]
    pub fn get_u_int64_list(self) -> Result<primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(12))
    }
    pub fn has_u_int64_list(&self) -> bool {
      !self.reader.get_pointer_field(12).is_null()
    }
    #[inline]
    pub fn get_float32_list(self) -> Result<primitive_list::Reader<'a,f32>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(13))
    }
    pub fn has_float32_list(&self) -> bool {
      !self.reader.get_pointer_field(13).is_null()
    }
    #[inline]
    pub fn get_float64_list(self) -> Result<primitive_list::Reader<'a,f64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(14))
    }
    pub fn has_float64_list(&self) -> bool {
      !self.reader.get_pointer_field(14).is_null()
    }
    #[inline]
    pub fn get_text_list(self) -> Result<text_list::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(15))
    }
    pub fn has_text_list(&self) -> bool {
      !self.reader.get_pointer_field(15).is_null()
    }
    #[inline]
    pub fn get_data_list(self) -> Result<data_list::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(16))
    }
    pub fn has_data_list(&self) -> bool {
      !self.reader.get_pointer_field(16).is_null()
    }
    #[inline]
    pub fn get_struct_list(self) -> Result<struct_list::Reader<'a,::test_capnp::test_all_types::Owned<>>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(17))
    }
    pub fn has_struct_list(&self) -> bool {
      !self.reader.get_pointer_field(17).is_null()
    }
    #[inline]
    pub fn get_enum_list(self) -> Result<enum_list::Reader<'a,::test_capnp::TestEnum>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(18))
    }
    pub fn has_enum_list(&self) -> bool {
      !self.reader.get_pointer_field(18).is_null()
    }
    #[inline]
    pub fn get_interface_list(self) -> Result<primitive_list::Reader<'a,()>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(19))
    }
    pub fn has_interface_list(&self) -> bool {
      !self.reader.get_pointer_field(19).is_null()
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    #[inline]
    fn struct_size() -> layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
    fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder: builder,  }
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>) -> Result<Builder<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(try!(builder.get_struct(_private::STRUCT_SIZE, ::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
    fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> Result<()> { pointer.set_struct(&value.reader, canonicalize) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn into_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }
    pub fn borrow<'b>(&'b mut self) -> Builder<'b,> {
      Builder { .. *self }
    }
    pub fn borrow_as_reader<'b>(&'b self) -> Reader<'b,> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.builder.into_reader().total_size()
    }
    #[inline]
    pub fn get_void_field(self) -> () {
      ()
    }
    #[inline]
    pub fn set_void_field(&mut self, _value: ())  {
    }
    #[inline]
    pub fn get_bool_field(self) -> bool {
      self.builder.get_bool_field(0)
    }
    #[inline]
    pub fn set_bool_field(&mut self, value: bool)  {
      self.builder.set_bool_field(0, value);
    }
    #[inline]
    pub fn get_int8_field(self) -> i8 {
      self.builder.get_data_field::<i8>(1)
    }
    #[inline]
    pub fn set_int8_field(&mut self, value: i8)  {
      self.builder.set_data_field::<i8>(1, value);
    }
    #[inline]
    pub fn get_int16_field(self) -> i16 {
      self.builder.get_data_field::<i16>(1)
    }
    #[inline]
    pub fn set_int16_field(&mut self, value: i16)  {
      self.builder.set_data_field::<i16>(1, value);
    }
    #[inline]
    pub fn get_int32_field(self) -> i32 {
      self.builder.get_data_field::<i32>(1)
    }
    #[inline]
    pub fn set_int32_field(&mut self, value: i32)  {
      self.builder.set_data_field::<i32>(1, value);
    }
    #[inline]
    pub fn get_int64_field(self) -> i64 {
      self.builder.get_data_field::<i64>(1)
    }
    #[inline]
    pub fn set_int64_field(&mut self, value: i64)  {
      self.builder.set_data_field::<i64>(1, value);
    }
    #[inline]
    pub fn get_u_int8_field(self) -> u8 {
      self.builder.get_data_field::<u8>(16)
    }
    #[inline]
    pub fn set_u_int8_field(&mut self, value: u8)  {
      self.builder.set_data_field::<u8>(16, value);
    }
    #[inline]
    pub fn get_u_int16_field(self) -> u16 {
      self.builder.get_data_field::<u16>(9)
    }
    #[inline]
    pub fn set_u_int16_field(&mut self, value: u16)  {
      self.builder.set_data_field::<u16>(9, value);
    }
    #[inline]
    pub fn get_u_int32_field(self) -> u32 {
      self.builder.get_data_field::<u32>(5)
    }
    #[inline]
    pub fn set_u_int32_field(&mut self, value: u32)  {
      self.builder.set_data_field::<u32>(5, value);
    }
    #[inline]
    pub fn get_u_int64_field(self) -> u64 {
      self.builder.get_data_field::<u64>(3)
    }
    #[inline]
    pub fn set_u_int64_field(&mut self, value: u64)  {
      self.builder.set_data_field::<u64>(3, value);
    }
    #[inline]
    pub fn get_float32_field(self) -> f32 {
      self.builder.get_data_field::<f32>(8)
    }
    #[inline]
    pub fn set_float32_field(&mut self, value: f32)  {
      self.builder.set_data_field::<f32>(8, value);
    }
    #[inline]
    pub fn get_float64_field(self) -> f64 {
      self.builder.get_data_field::<f64>(5)
    }
    #[inline]
    pub fn set_float64_field(&mut self, value: f64)  {
      self.builder.set_data_field::<f64>(5, value);
    }
    #[inline]
    pub fn get_text_field(self) -> Result<text::Builder<'a>> {
      self.builder.get_pointer_field(0).get_text(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_text_field(&mut self, value: text::Reader)  {
      self.builder.get_pointer_field(0).set_text(value);
    }
    #[inline]
    pub fn init_text_field(self, size: u32) -> text::Builder<'a> {
      self.builder.get_pointer_field(0).init_text(size)
    }
    pub fn has_text_field(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_data_field(self) -> Result<data::Builder<'a>> {
      self.builder.get_pointer_field(1).get_data(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_data_field(&mut self, value: data::Reader)  {
      self.builder.get_pointer_field(1).set_data(value);
    }
    #[inline]
    pub fn init_data_field(self, size: u32) -> data::Builder<'a> {
      self.builder.get_pointer_field(1).init_data(size)
    }
    pub fn has_data_field(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_struct_field(self) -> Result<::test_capnp::test_all_types::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(2))
    }
    #[inline]
    pub fn set_struct_field<'b>(&mut self, value: ::test_capnp::test_all_types::Reader<'b>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(2), value, false)
    }
    #[inline]
    pub fn init_struct_field(self, ) -> ::test_capnp::test_all_types::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(2), 0)
    }
    pub fn has_struct_field(&self) -> bool {
      !self.builder.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_enum_field(self) -> ::std::result::Result<::test_capnp::TestEnum,::capnp::NotInSchema> {
      ::capnp::traits::FromU16::from_u16(self.builder.get_data_field::<u16>(18))
    }
    #[inline]
    pub fn set_enum_field(&mut self, value: ::test_capnp::TestEnum)  {
      self.builder.set_data_field::<u16>(18, value as u16)
    }
    #[inline]
    pub fn get_interface_field(self) -> () {
      ()
    }
    #[inline]
    pub fn set_interface_field(&mut self, _value: ())  {
    }
    #[inline]
    pub fn get_void_list(self) -> Result<primitive_list::Builder<'a,()>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(3))
    }
    #[inline]
    pub fn set_void_list(&mut self, value: primitive_list::Reader<'a,()>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(3), value, false)
    }
    #[inline]
    pub fn init_void_list(self, size: u32) -> primitive_list::Builder<'a,()> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(3), size)
    }
    pub fn has_void_list(&self) -> bool {
      !self.builder.get_pointer_field(3).is_null()
    }
    #[inline]
    pub fn get_bool_list(self) -> Result<primitive_list::Builder<'a,bool>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(4))
    }
    #[inline]
    pub fn set_bool_list(&mut self, value: primitive_list::Reader<'a,bool>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(4), value, false)
    }
    #[inline]
    pub fn init_bool_list(self, size: u32) -> primitive_list::Builder<'a,bool> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(4), size)
    }
    pub fn has_bool_list(&self) -> bool {
      !self.builder.get_pointer_field(4).is_null()
    }
    #[inline]
    pub fn get_int8_list(self) -> Result<primitive_list::Builder<'a,i8>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(5))
    }
    #[inline]
    pub fn set_int8_list(&mut self, value: primitive_list::Reader<'a,i8>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(5), value, false)
    }
    #[inline]
    pub fn init_int8_list(self, size: u32) -> primitive_list::Builder<'a,i8> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(5), size)
    }
    pub fn has_int8_list(&self) -> bool {
      !self.builder.get_pointer_field(5).is_null()
    }
    #[inline]
    pub fn get_int16_list(self) -> Result<primitive_list::Builder<'a,i16>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(6))
    }
    #[inline]
    pub fn set_int16_list(&mut self, value: primitive_list::Reader<'a,i16>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(6), value, false)
    }
    #[inline]
    pub fn init_int16_list(self, size: u32) -> primitive_list::Builder<'a,i16> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(6), size)
    }
    pub fn has_int16_list(&self) -> bool {
      !self.builder.get_pointer_field(6).is_null()
    }
    #[inline]
    pub fn get_int32_list(self) -> Result<primitive_list::Builder<'a,i32>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(7))
    }
    #[inline]
    pub fn set_int32_list(&mut self, value: primitive_list::Reader<'a,i32>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(7), value, false)
    }
    #[inline]
    pub fn init_int32_list(self, size: u32) -> primitive_list::Builder<'a,i32> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(7), size)
    }
    pub fn has_int32_list(&self) -> bool {
      !self.builder.get_pointer_field(7).is_null()
    }
    #[inline]
    pub fn get_int64_list(self) -> Result<primitive_list::Builder<'a,i64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(8))
    }
    #[inline]
    pub fn set_int64_list(&mut self, value: primitive_list::Reader<'a,i64>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(8), value, false)
    }
    #[inline]
    pub fn init_int64_list(self, size: u32) -> primitive_list::Builder<'a,i64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(8), size)
    }
    pub fn has_int64_list(&self) -> bool {
      !self.builder.get_pointer_field(8).is_null()
    }
    #[inline]
    pub fn get_u_int8_list(self) -> Result<primitive_list::Builder<'a,u8>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(9))
    }
    #[inline]
    pub fn set_u_int8_list(&mut self, value: primitive_list::Reader<'a,u8>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(9), value, false)
    }
    #[inline]
    pub fn init_u_int8_list(self, size: u32) -> primitive_list::Builder<'a,u8> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(9), size)
    }
    pub fn has_u_int8_list(&self) -> bool {
      !self.builder.get_pointer_field(9).is_null()
    }
    #[inline]
    pub fn get_u_int16_list(self) -> Result<primitive_list::Builder<'a,u16>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(10))
    }
    #[inline]
    pub fn set_u_int16_list(&mut self, value: primitive_list::Reader<'a,u16>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(10), value, false)
    }
    #[inline]
    pub fn init_u_int16_list(self, size: u32) -> primitive_list::Builder<'a,u16> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(10), size)
    }
    pub fn has_u_int16_list(&self) -> bool {
      !self.builder.get_pointer_field(10).is_null()
    }
    #[inline]
    pub fn get_u_int32_list(self) -> Result<primitive_list::Builder<'a,u32>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(11))
    }
    #[inline]
    pub fn set_u_int32_list(&mut self, value: primitive_list::Reader<'a,u32>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(11), value, false)
    }
    #[inline]
    pub fn init_u_int32_list(self, size: u32) -> primitive_list::Builder<'a,u32> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(11), size)
    }
    pub fn has_u_int32_list(&self) -> bool {
      !self.builder.get_pointer_field(11).is_null()
    }
    #[inline]
    pub fn get_u_int64_list(self) -> Result<primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(12))
    }
    #[inline]
    pub fn set_u_int64_list(&mut self, value: primitive_list::Reader<'a,u64>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(12), value, false)
    }
    #[inline]
    pub fn init_u_int64_list(self, size: u32) -> primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(12), size)
    }
    pub fn has_u_int64_list(&self) -> bool {
      !self.builder.get_pointer_field(12).is_null()
    }
    #[inline]
    pub fn get_float32_list(self) -> Result<primitive_list::Builder<'a,f32>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(13))
    }
    #[inline]
    pub fn set_float32_list(&mut self, value: primitive_list::Reader<'a,f32>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(13), value, false)
    }
    #[inline]
    pub fn init_float32_list(self, size: u32) -> primitive_list::Builder<'a,f32> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(13), size)
    }
    pub fn has_float32_list(&self) -> bool {
      !self.builder.get_pointer_field(13).is_null()
    }
    #[inline]
    pub fn get_float64_list(self) -> Result<primitive_list::Builder<'a,f64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(14))
    }
    #[inline]
    pub fn set_float64_list(&mut self, value: primitive_list::Reader<'a,f64>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(14), value, false)
    }
    #[inline]
    pub fn init_float64_list(self, size: u32) -> primitive_list::Builder<'a,f64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(14), size)
    }
    pub fn has_float64_list(&self) -> bool {
      !self.builder.get_pointer_field(14).is_null()
    }
    #[inline]
    pub fn get_text_list(self) -> Result<text_list::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(15))
    }
    #[inline]
    pub fn set_text_list(&mut self, value: text_list::Reader<'a>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(15), value, false)
    }
    #[inline]
    pub fn init_text_list(self, size: u32) -> text_list::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(15), size)
    }
    pub fn has_text_list(&self) -> bool {
      !self.builder.get_pointer_field(15).is_null()
    }
    #[inline]
    pub fn get_data_list(self) -> Result<data_list::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(16))
    }
    #[inline]
    pub fn set_data_list(&mut self, value: data_list::Reader<'a>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(16), value, false)
    }
    #[inline]
    pub fn init_data_list(self, size: u32) -> data_list::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(16), size)
    }
    pub fn has_data_list(&self) -> bool {
      !self.builder.get_pointer_field(16).is_null()
    }
    #[inline]
    pub fn get_struct_list(self) -> Result<struct_list::Builder<'a,::test_capnp::test_all_types::Owned<>>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(17))
    }
    #[inline]
    pub fn set_struct_list(&mut self, value: struct_list::Reader<'a,::test_capnp::test_all_types::Owned<>>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(17), value, false)
    }
    #[inline]
    pub fn init_struct_list(self, size: u32) -> struct_list::Builder<'a,::test_capnp::test_all_types::Owned<>> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(17), size)
    }
    pub fn has_struct_list(&self) -> bool {
      !self.builder.get_pointer_field(17).is_null()
    }
    #[inline]
    pub fn get_enum_list(self) -> Result<enum_list::Builder<'a,::test_capnp::TestEnum>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(18))
    }
    #[inline]
    pub fn set_enum_list(&mut self, value: enum_list::Reader<'a,::test_capnp::TestEnum>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(18), value, false)
    }
    #[inline]
    pub fn init_enum_list(self, size: u32) -> enum_list::Builder<'a,::test_capnp::TestEnum> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(18), size)
    }
    pub fn has_enum_list(&self) -> bool {
      !self.builder.get_pointer_field(18).is_null()
    }
    #[inline]
    pub fn get_interface_list(self) -> Result<primitive_list::Builder<'a,()>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(19))
    }
    #[inline]
    pub fn set_interface_list(&mut self, value: primitive_list::Reader<'a,()>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(19), value, false)
    }
    #[inline]
    pub fn init_interface_list(self, size: u32) -> primitive_list::Builder<'a,()> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(19), size)
    }
    pub fn has_interface_list(&self) -> bool {
      !self.builder.get_pointer_field(19).is_null()
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
    pub fn get_struct_field(&self) -> ::test_capnp::test_all_types::Pipeline<> {
      FromTypelessPipeline::new(self._typeless.get_pointer_field(2))
    }
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 6, pointers: 20 };
    pub const TYPE_ID: u64 = 0xe3a3ab2716ba0ee3;
  }
}
