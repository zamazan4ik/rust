use super::{mir::Mutability, mir::Safety, with, DefId};
use crate::rustc_internal::Opaque;

#[derive(Copy, Clone, Debug)]
pub struct Ty(pub usize);

impl Ty {
    pub fn kind(&self) -> TyKind {
        with(|context| context.ty_kind(*self))
    }
}

pub(crate) type Const = Opaque;
type Ident = Opaque;
pub(crate) type Region = Opaque;
type Span = Opaque;

#[derive(Clone, Debug)]
pub enum TyKind {
    RigidTy(RigidTy),
    Alias(AliasKind, AliasTy),
    Param(ParamTy),
    Bound(usize, BoundTy),
}

#[derive(Clone, Debug)]
pub enum RigidTy {
    Bool,
    Char,
    Int(IntTy),
    Uint(UintTy),
    Float(FloatTy),
    Adt(AdtDef, GenericArgs),
    Foreign(ForeignDef),
    Str,
    Array(Ty, Const),
    Slice(Ty),
    RawPtr(Ty, Mutability),
    Ref(Region, Ty, Mutability),
    FnDef(FnDef, GenericArgs),
    FnPtr(PolyFnSig),
    Closure(ClosureDef, GenericArgs),
    Generator(GeneratorDef, GenericArgs, Movability),
    Dynamic(Vec<Binder<ExistentialPredicate>>, Region, DynKind),
    Never,
    Tuple(Vec<Ty>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntTy {
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UintTy {
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FloatTy {
    F32,
    F64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Movability {
    Static,
    Movable,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ForeignDef(pub(crate) DefId);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FnDef(pub(crate) DefId);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClosureDef(pub(crate) DefId);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GeneratorDef(pub(crate) DefId);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ParamDef(pub(crate) DefId);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BrNamedDef(pub(crate) DefId);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AdtDef(pub(crate) DefId);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AliasDef(pub(crate) DefId);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TraitDef(pub(crate) DefId);

impl TraitDef {
    pub fn trait_decl(&self) -> TraitDecl {
        with(|cx| cx.trait_decl(self))
    }
}

#[derive(Clone, Debug)]
pub struct GenericArgs(pub Vec<GenericArgKind>);

#[derive(Clone, Debug)]
pub enum GenericArgKind {
    Lifetime(Region),
    Type(Ty),
    Const(Const),
}

#[derive(Clone, Debug)]
pub enum TermKind {
    Type(Ty),
    Const(Const),
}

#[derive(Clone, Debug)]
pub enum AliasKind {
    Projection,
    Inherent,
    Opaque,
    Weak,
}

#[derive(Clone, Debug)]
pub struct AliasTy {
    pub def_id: AliasDef,
    pub args: GenericArgs,
}

pub type PolyFnSig = Binder<FnSig>;

#[derive(Clone, Debug)]
pub struct FnSig {
    pub inputs_and_output: Vec<Ty>,
    pub c_variadic: bool,
    pub unsafety: Safety,
    pub abi: Abi,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Abi {
    Rust,
    C { unwind: bool },
    Cdecl { unwind: bool },
    Stdcall { unwind: bool },
    Fastcall { unwind: bool },
    Vectorcall { unwind: bool },
    Thiscall { unwind: bool },
    Aapcs { unwind: bool },
    Win64 { unwind: bool },
    SysV64 { unwind: bool },
    PtxKernel,
    Msp430Interrupt,
    X86Interrupt,
    AmdGpuKernel,
    EfiApi,
    AvrInterrupt,
    AvrNonBlockingInterrupt,
    CCmseNonSecureCall,
    Wasm,
    System { unwind: bool },
    RustIntrinsic,
    RustCall,
    PlatformIntrinsic,
    Unadjusted,
    RustCold,
}

#[derive(Clone, Debug)]
pub struct Binder<T> {
    pub value: T,
    pub bound_vars: Vec<BoundVariableKind>,
}

#[derive(Clone, Debug)]
pub enum BoundVariableKind {
    Ty(BoundTyKind),
    Region(BoundRegionKind),
    Const,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BoundTyKind {
    Anon,
    Param(ParamDef, String),
}

#[derive(Clone, Debug)]
pub enum BoundRegionKind {
    BrAnon(Option<Span>),
    BrNamed(BrNamedDef, String),
    BrEnv,
}

#[derive(Clone, Debug)]
pub enum DynKind {
    Dyn,
    DynStar,
}

#[derive(Clone, Debug)]
pub enum ExistentialPredicate {
    Trait(ExistentialTraitRef),
    Projection(ExistentialProjection),
    AutoTrait(TraitDef),
}

#[derive(Clone, Debug)]
pub struct ExistentialTraitRef {
    pub def_id: TraitDef,
    pub generic_args: GenericArgs,
}

#[derive(Clone, Debug)]
pub struct ExistentialProjection {
    pub def_id: TraitDef,
    pub generic_args: GenericArgs,
    pub term: TermKind,
}

#[derive(Clone, Debug)]
pub struct ParamTy {
    pub index: u32,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct BoundTy {
    pub var: usize,
    pub kind: BoundTyKind,
}

pub type Bytes = Vec<Option<u8>>;
pub type Size = usize;
pub type Prov = Opaque;
pub type Align = u64;
pub type InitMaskMaterialized = Vec<u64>;

/// Stores the provenance information of pointers stored in memory.
#[derive(Clone, Debug)]
pub struct ProvenanceMap {
    /// Provenance in this map applies from the given offset for an entire pointer-size worth of
    /// bytes. Two entries in this map are always at least a pointer size apart.
    pub ptrs: Vec<(Size, Prov)>,
}

#[derive(Clone, Debug)]
pub struct Allocation {
    pub bytes: Bytes,
    pub provenance: ProvenanceMap,
    pub align: Align,
    pub mutability: Mutability,
}

pub enum TraitSpecializationKind {
    None,
    Marker,
    AlwaysApplicable,
}

pub struct TraitDecl {
    pub def_id: TraitDef,
    pub unsafety: Safety,
    pub paren_sugar: bool,
    pub has_auto_impl: bool,
    pub is_marker: bool,
    pub is_coinductive: bool,
    pub skip_array_during_method_dispatch: bool,
    pub specialization_kind: TraitSpecializationKind,
    pub must_implement_one_of: Option<Vec<Ident>>,
    pub implement_via_object: bool,
    pub deny_explicit_impl: bool,
}
