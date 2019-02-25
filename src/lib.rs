#![allow(non_upper_case_globals)]
use alembic_sys::*;
use std::{ffi::c_void, marker::PhantomData, mem, os::raw::c_char, path::Path, ptr};

//--------------------------------------------------------------------------------------------------
struct IFactory(Alembic_AbcCoreFactory_v11_IFactory);

impl IFactory {
    pub fn new() -> IFactory {
        unsafe {
            let mut ifactory: Alembic_AbcCoreFactory_v11_IFactory = mem::uninitialized();
            Alembic_AbcCoreFactory_v11_IFactory_new(&mut ifactory as *mut _);
            IFactory(ifactory)
        }
    }
}

impl Drop for IFactory {
    fn drop(&mut self) {
        unsafe { Alembic_AbcCoreFactory_v11_IFactory_dtor(&mut self.0) }
    }
}

//--------------------------------------------------------------------------------------------------
// ARCHIVE
struct IArchive(
    *mut Alembic_AbcCoreAbstract_v11_ArchiveReader,
    Alembic_AbcCoreAbstract_v11_ArchiveReaderPtr,
);

impl IArchive {
    /*fn get(&self) -> *Alembic_AbcCoreAbstract_v11_ArchiveReader {
        Alembic_AbcCoreAbstract_v11_ArchiveReaderPtr_get()
    }
    fn get_mut(&self) -> *Alembic_AbcCoreAbstract_v11_ArchiveReader {
    }*/

    pub fn from_file<P: AsRef<Path>>(path: P) -> IArchive {
        unsafe {
            let mut factory = IFactory::new();
            let mut ty: Alembic_AbcCoreFactory_v11_IFactory_CoreType = mem::uninitialized();
            let mut sharedptr: Alembic_AbcCoreAbstract_v11_ArchiveReaderPtr = mem::uninitialized();
            let ptr = Alembic_AbcCoreFactory_v11_IFactory_getArchive(
                &mut factory.0,
                path.as_ref().as_stringref(),
                &mut ty,
                &mut sharedptr,
            );
            IArchive(ptr, sharedptr)
        }
    }

    pub fn name(&self) -> &str {
        unsafe {
            Alembic_AbcCoreAbstract_v11_ArchiveReader_getName(self.0)
                .try_into_str()
                .unwrap()
        }
    }

    pub fn top(&self) -> IObject {
        unsafe {
            let mut sharedptr: Alembic_AbcCoreAbstract_v11_ObjectReaderPtr = mem::uninitialized();
            let ptr = Alembic_AbcCoreAbstract_v11_ArchiveReader_getTop(self.0, &mut sharedptr);
            IObject(ptr, sharedptr)
        }
    }
}

//--------------------------------------------------------------------------------------------------
// TYPES
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PodType {
    Boolean = Alembic_Util_v11_PlainOldDataType_kBooleanPOD,
    Uint8 = Alembic_Util_v11_PlainOldDataType_kUint8POD,
    Int8 = Alembic_Util_v11_PlainOldDataType_kInt8POD,
    Uint16 = Alembic_Util_v11_PlainOldDataType_kUint16POD,
    Int16 = Alembic_Util_v11_PlainOldDataType_kInt16POD,
    Uint32 = Alembic_Util_v11_PlainOldDataType_kUint32POD,
    Int32 = Alembic_Util_v11_PlainOldDataType_kInt32POD,
    Uint64 = Alembic_Util_v11_PlainOldDataType_kUint64POD,
    Int64 = Alembic_Util_v11_PlainOldDataType_kInt64POD,
    Float16 = Alembic_Util_v11_PlainOldDataType_kFloat16POD,
    Float32 = Alembic_Util_v11_PlainOldDataType_kFloat32POD,
    Float64 = Alembic_Util_v11_PlainOldDataType_kFloat64POD,
    String = Alembic_Util_v11_PlainOldDataType_kStringPOD,
    Wstring = Alembic_Util_v11_PlainOldDataType_kWstringPOD,
    Unknown = Alembic_Util_v11_PlainOldDataType_kUnknownPOD,
}

impl From<Alembic_Util_v11_PlainOldDataType> for PodType {
    fn from(raw: Alembic_Util_v11_PlainOldDataType) -> PodType {
        match raw {
            Alembic_Util_v11_PlainOldDataType_kBooleanPOD => PodType::Boolean,
            Alembic_Util_v11_PlainOldDataType_kUint8POD => PodType::Uint8,
            Alembic_Util_v11_PlainOldDataType_kInt8POD => PodType::Int8,
            Alembic_Util_v11_PlainOldDataType_kUint16POD => PodType::Uint16,
            Alembic_Util_v11_PlainOldDataType_kInt16POD => PodType::Int16,
            Alembic_Util_v11_PlainOldDataType_kUint32POD => PodType::Uint32,
            Alembic_Util_v11_PlainOldDataType_kInt32POD => PodType::Int32,
            Alembic_Util_v11_PlainOldDataType_kUint64POD => PodType::Uint64,
            Alembic_Util_v11_PlainOldDataType_kInt64POD => PodType::Int64,
            Alembic_Util_v11_PlainOldDataType_kFloat16POD => PodType::Float16,
            Alembic_Util_v11_PlainOldDataType_kFloat32POD => PodType::Float32,
            Alembic_Util_v11_PlainOldDataType_kFloat64POD => PodType::Float64,
            Alembic_Util_v11_PlainOldDataType_kStringPOD => PodType::String,
            Alembic_Util_v11_PlainOldDataType_kWstringPOD => PodType::Wstring,
            Alembic_Util_v11_PlainOldDataType_kUnknownPOD => PodType::Unknown,
            _ => unimplemented!("unimplemented PlainOldDataType"),
        }
    }
}

/// New type here because it's public.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DataType(PodType, u8);

impl DataType {
    unsafe fn from_raw(datatype: *const Alembic_AbcCoreAbstract_v11_DataType) -> Self {
        DataType((*datatype).m_pod.into(), (*datatype).m_extent)
    }
}

pub unsafe trait PropertyElementType: Copy {
    const DATA_TYPE: DataType;
}

pub trait PropertyTraits {
    type Element: PropertyElementType;
    const INTERPRETATION: &'static str;
}

// Interpretation + Type
// if datatype matches, then can cast to

macro_rules! impl_property_type {
    ($t:ty, $pod:expr, $ext:expr) => {
        unsafe impl PropertyElementType for $t {
            const DATA_TYPE: DataType = DataType($pod, $ext);
        }
    };
}

// TODO Float16 support (`half` crate)
// TODO Strings
impl_property_type!(u8, PodType::Uint8, 1);
impl_property_type!(u16, PodType::Uint16, 1);
impl_property_type!(u32, PodType::Uint32, 1);
impl_property_type!(u64, PodType::Uint64, 1);
impl_property_type!(i8, PodType::Int8, 1);
impl_property_type!(i16, PodType::Int16, 1);
impl_property_type!(i32, PodType::Int32, 1);
impl_property_type!(i64, PodType::Int64, 1);
impl_property_type!(f32, PodType::Float32, 1);
impl_property_type!(f64, PodType::Float64, 1);

impl_property_type!([u8; 2], PodType::Uint8, 2);
impl_property_type!([u16; 2], PodType::Uint16, 2);
impl_property_type!([u32; 2], PodType::Uint32, 2);
impl_property_type!([u64; 2], PodType::Uint64, 2);
impl_property_type!([i8; 2], PodType::Int8, 2);
impl_property_type!([i16; 2], PodType::Int16, 2);
impl_property_type!([i32; 2], PodType::Int32, 2);
impl_property_type!([i64; 2], PodType::Int64, 2);
impl_property_type!([f32; 2], PodType::Float32, 2);
impl_property_type!([f64; 2], PodType::Float64, 2);

impl_property_type!([u8; 3], PodType::Uint8, 3);
impl_property_type!([u16; 3], PodType::Uint16, 3);
impl_property_type!([u32; 3], PodType::Uint32, 3);
impl_property_type!([u64; 3], PodType::Uint64, 3);
impl_property_type!([i8; 3], PodType::Int8, 3);
impl_property_type!([i16; 3], PodType::Int16, 3);
impl_property_type!([i32; 3], PodType::Int32, 3);
impl_property_type!([i64; 3], PodType::Int64, 3);
impl_property_type!([f32; 3], PodType::Float32, 3);
impl_property_type!([f64; 3], PodType::Float64, 3);

impl_property_type!([u8; 4], PodType::Uint8, 4);
impl_property_type!([u16; 4], PodType::Uint16, 4);
impl_property_type!([u32; 4], PodType::Uint32, 4);
impl_property_type!([u64; 4], PodType::Uint64, 4);
impl_property_type!([i8; 4], PodType::Int8, 4);
impl_property_type!([i16; 4], PodType::Int16, 4);
impl_property_type!([i32; 4], PodType::Int32, 4);
impl_property_type!([i64; 4], PodType::Int64, 4);
impl_property_type!([f32; 4], PodType::Float32, 4);
impl_property_type!([f64; 4], PodType::Float64, 4);

//--------------------------------------------------------------------------------------------------
// SAMPLES

/// Untyped, owned array sample
pub struct ArraySample(
    *mut Alembic_AbcCoreAbstract_v11_ArraySample,
    Alembic_AbcCoreAbstract_v11_ArraySamplePtr,
);

impl ArraySample {
    /// Total number of elements.
    /// This entails some computation, so call it once.
    /// Not usize because Alembic uses u64.
    pub fn len(&self) -> u64 {
        let dims = self.dimensions();
        if dims.len() == 0 {
            0
        } else {
            dims.iter().fold(1, |acc, &d| acc * d)
        }
    }

    pub fn dimensions(&self) -> &[u64] {
        unsafe {
            let raw = Alembic_AbcCoreAbstract_v11_ArraySample_dimensions(self.0);
            std::slice::from_raw_parts(raw.ptr, raw.len)
        }
    }

    pub fn data_type(&self) -> DataType {
        unsafe { DataType::from_raw(Alembic_AbcCoreAbstract_v11_ArraySample_getDataType(self.0)) }
    }

    pub fn data_raw(&self) -> *const c_void {
        unsafe { Alembic_AbcCoreAbstract_v11_ArraySample_getData(self.0) }
    }

    pub fn into_typed<T: PropertyElementType>(self) -> Option<TypedArraySample<T>> {
        if self.data_type() == T::DATA_TYPE {
            Some(TypedArraySample(self, PhantomData))
        } else {
            None
        }
    }
}

/// Owned typed array sample
/// Can be cast into any PropertyType that has the same DataType
pub struct TypedArraySample<Element: PropertyElementType>(ArraySample, PhantomData<Element>);

impl<Element: PropertyElementType> TypedArraySample<Element> {
    pub fn data(&self) -> &[Element] {
        unsafe {
            let len = self.0.len() as usize;
            let ptr = self.0.data_raw() as *const Element;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

//--------------------------------------------------------------------------------------------------
// PROPERTIES

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PropertyType {
    Compound = Alembic_AbcCoreAbstract_v11_PropertyType_kCompoundProperty,
    Scalar = Alembic_AbcCoreAbstract_v11_PropertyType_kScalarProperty,
    Array = Alembic_AbcCoreAbstract_v11_PropertyType_kArrayProperty,
}

impl From<Alembic_Util_v11_PlainOldDataType> for PropertyType {
    fn from(raw: Alembic_AbcCoreAbstract_v11_PropertyType) -> PropertyType {
        match raw {
            Alembic_AbcCoreAbstract_v11_PropertyType_kCompoundProperty => PropertyType::Compound,
            Alembic_AbcCoreAbstract_v11_PropertyType_kScalarProperty => PropertyType::Scalar,
            Alembic_AbcCoreAbstract_v11_PropertyType_kArrayProperty => PropertyType::Array,
            _ => unimplemented!("unimplemented PropertyType"),
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct PropertyHeader(Alembic_AbcCoreAbstract_v11_PropertyHeader);

impl PropertyHeader {
    pub fn name(&self) -> &str {
        unsafe {
            Alembic_AbcCoreAbstract_v11_PropertyHeader_getName(&self.0)
                .try_into_str()
                .unwrap()
        }
    }

    pub fn ty(&self) -> PropertyType {
        unsafe { Alembic_AbcCoreAbstract_v11_PropertyHeader_getPropertyType(&self.0).into() }
    }
}

/// A property.
/// Properties are one of three kinds: Scalar, Component, or Array.
/// Use the as_* methods to downcast a IPropertyBase into a more concrete type.
pub struct IPropertyBase(
    *mut Alembic_AbcCoreAbstract_v11_BasePropertyReader,
    Alembic_AbcCoreAbstract_v11_BasePropertyReaderPtr,
);

impl IPropertyBase {
    /// Properties are created with a collection of metadata that is stored
    /// in a lightweight structure called PropertyHeader.
    /// This returns a constant reference to the PropertyHeader which
    /// was given upon creation.
    pub fn header(&self) -> &PropertyHeader {
        unsafe {
            let raw = Alembic_AbcCoreAbstract_v11_BasePropertyReader_getHeader(self.0);
            &*(raw as *const PropertyHeader)
        }
    }

    /// Returns true if this property is a scalar property.
    pub fn is_scalar(&self) -> bool {
        self.header().ty() == PropertyType::Scalar
    }

    /// Returns true if this property is a compound property.
    pub fn is_compound(&self) -> bool {
        self.header().ty() == PropertyType::Compound
    }

    /// Returns true if this property is an array property.
    pub fn is_array(&self) -> bool {
        self.header().ty() == PropertyType::Array
    }

    pub fn into_array(self) -> Option<IArrayProperty> {
        unsafe {
            let mut sharedptr = mem::uninitialized();
            let ptr =
                Alembic_AbcCoreAbstract_v11_BasePropertyReader_asArrayPtr(self.0, &mut sharedptr);
            if ptr != ptr::null_mut() {
                Some(IArrayProperty(ptr, sharedptr))
            } else {
                None
            }
        }
    }

    pub fn into_scalar(self) -> Option<IScalarProperty> {
        unsafe {
            let mut sharedptr = mem::uninitialized();
            let ptr =
                Alembic_AbcCoreAbstract_v11_BasePropertyReader_asScalarPtr(self.0, &mut sharedptr);
            if ptr != ptr::null_mut() {
                Some(IScalarProperty(ptr, sharedptr))
            } else {
                None
            }
        }
    }

    pub fn into_compound(self) -> Option<ICompoundProperty> {
        unsafe {
            let mut sharedptr = mem::uninitialized();
            let ptr = Alembic_AbcCoreAbstract_v11_BasePropertyReader_asCompoundPtr(
                self.0,
                &mut sharedptr,
            );
            if ptr != ptr::null_mut() {
                Some(ICompoundProperty(ptr, sharedptr))
            } else {
                None
            }
        }
    }
}

// A scalar property.
pub struct IScalarProperty(
    *mut Alembic_AbcCoreAbstract_v11_ScalarPropertyReader,
    Alembic_AbcCoreAbstract_v11_ScalarPropertyReaderPtr,
);

impl IScalarProperty {}

/// An array property, of varying size and rank.
pub struct IArrayProperty(
    *mut Alembic_AbcCoreAbstract_v11_ArrayPropertyReader,
    Alembic_AbcCoreAbstract_v11_ArrayPropertyReaderPtr,
);

impl IArrayProperty {}

/// Represents a typed array property. Element is the type of the elements of the array.
pub struct ITypedArrayProperty<Element: PropertyElementType>(IArrayProperty, PhantomData<Element>);

impl<Element: PropertyElementType> ITypedArrayProperty<Element> {}

/// A CompoundProperty is a group of other properties, possibly Simple
/// or possibly Compound. Every object has at one of these.
pub struct ICompoundProperty(
    *mut Alembic_AbcCoreAbstract_v11_CompoundPropertyReader,
    Alembic_AbcCoreAbstract_v11_CompoundPropertyReaderPtr,
);

impl ICompoundProperty {
    /// Returns the header of a component property given its index.
    /// Panics if the index is out-of-bounds.
    ///
    /// TODO actually panic
    pub fn property_header_by_index(&self, index: usize) -> &PropertyHeader {
        unsafe {
            let raw = Alembic_AbcCoreAbstract_v11_CompoundPropertyReader_getPropertyHeaderByIndex(
                self.0, index,
            );
            &*(raw as *const PropertyHeader)
        }
    }

    /// Returns an iterator over the headers of all component properties.
    pub fn property_headers<'a>(&'a self) -> impl Iterator<Item = &'a PropertyHeader> + 'a {
        let len =
            unsafe { Alembic_AbcCoreAbstract_v11_CompoundPropertyReader_getNumProperties(self.0) };
        (0..len).map(move |i| self.property_header_by_index(i))
    }

    /// See [IProperty::header].
    pub fn header(&self) -> &PropertyHeader {
        unsafe {
            let raw = Alembic_AbcCoreAbstract_v11_BasePropertyReader_getHeader(&(*self.0)._base);
            &*(raw as *const PropertyHeader)
        }
    }
}

//--------------------------------------------------------------------------------------------------
// OBJECTS

// Note: layout is known, but should always be used through an opaque pointer
#[derive(Debug)]
#[repr(transparent)]
struct ObjectHeader(Alembic_AbcCoreAbstract_v11_ObjectHeader);

impl ObjectHeader {
    pub fn name(&self) -> &str {
        unsafe {
            Alembic_AbcCoreAbstract_v11_ObjectHeader_getName(&self.0)
                .try_into_str()
                .unwrap()
        }
    }
}

struct IObject(
    *mut Alembic_AbcCoreAbstract_v11_ObjectReader,
    Alembic_AbcCoreAbstract_v11_ObjectReaderPtr,
);

impl IObject {
    pub fn header(&self) -> &ObjectHeader {
        unsafe {
            let hdr = Alembic_AbcCoreAbstract_v11_ObjectReader_getHeader(self.0);
            &*(hdr as *const ObjectHeader)
        }
    }

    pub fn name(&self) -> &str {
        self.header().name()
    }

    pub fn child_headers<'a>(&'a self) -> impl Iterator<Item = &'a ObjectHeader> + 'a {
        let len = unsafe { Alembic_AbcCoreAbstract_v11_ObjectReader_getNumChildren(self.0) };
        (0..len).map(move |i| self.child_header_by_index(i))
    }

    pub fn child_header_by_index(&self, index: usize) -> &ObjectHeader {
        unsafe {
            let hdr = Alembic_AbcCoreAbstract_v11_ObjectReader_getChildHeaderByIndex(self.0, index);
            &*(hdr as *const ObjectHeader)
        }
    }

    pub fn properties(&self) -> ICompoundProperty {
        unsafe {
            let mut sharedptr = mem::uninitialized();
            let ptr =
                Alembic_AbcCoreAbstract_v11_ObjectReader_getProperties(self.0, &mut sharedptr);
            ICompoundProperty(ptr, sharedptr)
        }
    }

    /*pub fn child(&self, name: &str) {
        unsafe {
            Alembic_AbcCoreAbstract_v11_ObjectReader_getChildByName(self.0,
        }
    }*/
}

//--------------------------------------------------------------------------------------------------
pub struct IPolyMesh {
    object: IObject,
    positions: ITypedArrayProperty<[f32; 3]>,
    velocities: ITypedArrayProperty<[f32; 3]>,
    indices: ITypedArrayProperty<i32>,
    counts: ITypedArrayProperty<i32>,
}

impl IPolyMesh {
    pub fn new(parent: IObject) {}
}

pub struct IFaceSet {
    object: IObject,
}

// GeomParam: values property + indices property (optional)

//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::IArchive;

    #[test]
    fn compiles() {
        let arc = IArchive::from_file("../tests/data/alembic_octopus.abc");
        let name = arc.name();
        eprintln!("top object = {}", arc.top().name());
        let top = arc.top();
        let children = top.child_headers().collect::<Vec<_>>();
        eprintln!("children = {:?}", children);
        let props = top.properties();
        let props = props
            .property_headers()
            .map(|p| p.name())
            .collect::<Vec<_>>();
        eprintln!("properties = {:?}", props);

        // open archive DONE
        // get top object DONE
        // recursively traverse children
        // get/read various properties
        // samples
        // implement schemas (XForm, PolyGeom)
        // IObject::try_into_schema<T: Schema>(self) -> T
        //
        // Q: Schema own objects or borrow?
        // original library copies compound property ptr (copy shared_ptr)
        // into() or as()?
        // don't copy, as it would be good to avoid incrementing the c++ refcount
        // move, or borrow?
        // move, and deref to IObject?
        // note that objects don't even borrow the parent (don't need to)
        // IObject is not clone, but easy to have two versions (just call getChild twice)
        // clone() IObject, then into() OR directly into() to avoid refcount++
        //
        // return &IObject? -> must allocate IObject somewhere
        // return IObject<'a>? Borrow archive? maybe but it's a gratuitous restriction of the underlying API

        // -> impedance mismatch: if the C++ library is refcounted, don't work around it

        // Refactor:
        // pub mod input
        //      (archive)
        //      (object)
        //      (objectheader)
        //      pub mod factory
        //          (factory)
        //      pub mod properties
        //          (propertyheader)
        //          (property)
        //      pub mod geom
        //          (mod polymesh)
        //          (mod xform)
        //          (mod faceset)
    }
}
