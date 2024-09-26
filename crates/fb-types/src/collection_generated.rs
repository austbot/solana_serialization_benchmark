// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use crate::state_generated::*;
use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum CollectionTypesOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct CollectionTypes<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for CollectionTypes<'a> {
  type Inner = CollectionTypes<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> CollectionTypes<'a> {
  pub const VT_VEC_PUBLIC_KEY: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    CollectionTypes { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args CollectionTypesArgs<'args>
  ) -> flatbuffers::WIPOffset<CollectionTypes<'bldr>> {
    let mut builder = CollectionTypesBuilder::new(_fbb);
    if let Some(x) = args.vec_public_key { builder.add_vec_public_key(x); }
    builder.finish()
  }

  pub fn unpack(&self) -> CollectionTypesT {
    let vec_public_key = self.vec_public_key().map(|x| {
      x.iter().map(|t| t.unpack()).collect()
    });
    CollectionTypesT {
      vec_public_key,
    }
  }

  #[inline]
  pub fn vec_public_key(&self) -> Option<flatbuffers::Vector<'a, PublicKey>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, PublicKey>>>(CollectionTypes::VT_VEC_PUBLIC_KEY, None)}
  }
}

impl flatbuffers::Verifiable for CollectionTypes<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, PublicKey>>>("vec_public_key", Self::VT_VEC_PUBLIC_KEY, false)?
     .finish();
    Ok(())
  }
}
pub struct CollectionTypesArgs<'a> {
    pub vec_public_key: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, PublicKey>>>,
}
impl<'a> Default for CollectionTypesArgs<'a> {
  #[inline]
  fn default() -> Self {
    CollectionTypesArgs {
      vec_public_key: None,
    }
  }
}

pub struct CollectionTypesBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> CollectionTypesBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_vec_public_key(&mut self, vec_public_key: flatbuffers::WIPOffset<flatbuffers::Vector<'b , PublicKey>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(CollectionTypes::VT_VEC_PUBLIC_KEY, vec_public_key);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> CollectionTypesBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    CollectionTypesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<CollectionTypes<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for CollectionTypes<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("CollectionTypes");
      ds.field("vec_public_key", &self.vec_public_key());
      ds.finish()
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CollectionTypesT {
  pub vec_public_key: Option<Vec<PublicKeyT>>,
}
impl Default for CollectionTypesT {
  fn default() -> Self {
    Self {
      vec_public_key: None,
    }
  }
}
impl CollectionTypesT {
  pub fn pack<'b, A: flatbuffers::Allocator + 'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b, A>
  ) -> flatbuffers::WIPOffset<CollectionTypes<'b>> {
    let vec_public_key = self.vec_public_key.as_ref().map(|x|{
      let w: Vec<_> = x.iter().map(|t| t.pack()).collect();_fbb.create_vector(&w)
    });
    CollectionTypes::create(_fbb, &CollectionTypesArgs{
      vec_public_key,
    })
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `CollectionTypes`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_collection_types_unchecked`.
pub fn root_as_collection_types(buf: &[u8]) -> Result<CollectionTypes, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<CollectionTypes>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `CollectionTypes` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_collection_types_unchecked`.
pub fn size_prefixed_root_as_collection_types(buf: &[u8]) -> Result<CollectionTypes, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<CollectionTypes>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `CollectionTypes` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_collection_types_unchecked`.
pub fn root_as_collection_types_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<CollectionTypes<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<CollectionTypes<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `CollectionTypes` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_collection_types_unchecked`.
pub fn size_prefixed_root_as_collection_types_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<CollectionTypes<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<CollectionTypes<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a CollectionTypes and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `CollectionTypes`.
pub unsafe fn root_as_collection_types_unchecked(buf: &[u8]) -> CollectionTypes {
  flatbuffers::root_unchecked::<CollectionTypes>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed CollectionTypes and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `CollectionTypes`.
pub unsafe fn size_prefixed_root_as_collection_types_unchecked(buf: &[u8]) -> CollectionTypes {
  flatbuffers::size_prefixed_root_unchecked::<CollectionTypes>(buf)
}
#[inline]
pub fn finish_collection_types_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<CollectionTypes<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_collection_types_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<CollectionTypes<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
