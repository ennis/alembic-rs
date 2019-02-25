#include <Alembic/Abc/All.h>
#include <Alembic/AbcCoreAbstract/All.h>
#include <Alembic/AbcCoreHDF5/All.h>
#include <Alembic/AbcCoreOgawa/All.h>
#include <Alembic/AbcCoreFactory/All.h>
#include <Alembic/AbcCollection/All.h>
#include <Alembic/AbcGeom/All.h>
#include <Alembic/AbcMaterial/All.h>
#include <Alembic/AbcCoreFactory/IFactory.h>

namespace Abc = ::Alembic::Abc;
namespace AbcA = ::Alembic::AbcCoreAbstract;
namespace AbcF = ::Alembic::AbcCoreFactory;
namespace AbcG = ::Alembic::AbcGeom;
namespace AbcU = ::Alembic::Util;

using AbcA::index_t;


extern "C" {

struct CStringRef
{
    size_t len;
    const char *ptr;
};

struct SliceUsize
{
    size_t len;
    const size_t* ptr;
};

struct SliceU64
{
    size_t len;
    const uint64_t* ptr;
};

CStringRef Util_getStdString(const std::string* string);

/*AbcA::ArchiveReader *
Alembic_AbcCoreAbstract_v11_ArchiveReaderPtr_get(AbcA::ArchiveReaderPtr *thisPtr_);

const AbcA::ArchiveReader *
Alembic_AbcCoreAbstract_v11_ArchiveReaderPtr_get_const(const AbcA::ArchiveReaderPtr *thisPtr_);*/

CStringRef
Alembic_AbcCoreAbstract_v11_ArchiveReader_getName(AbcA::ArchiveReader *this_);

AbcA::ObjectReader*
Alembic_AbcCoreAbstract_v11_ArchiveReader_getTop(AbcA::ArchiveReader *this_, AbcA::ObjectReaderPtr *oObjectReaderPtr);

void
Alembic_AbcCoreAbstract_v11_ArchiveReaderPtr_dtor(AbcA::ArchiveReaderPtr* thisPtr_);

const AbcA::ObjectHeader *
Alembic_AbcCoreAbstract_v11_ObjectReader_getHeader(const AbcA::ObjectReader *this_);

CStringRef
Alembic_AbcCoreAbstract_v11_ObjectReader_getName(const AbcA::ObjectReader *this_);

const AbcA::MetaData *
Alembic_AbcCoreAbstract_v11_ObjectReader_getMetaData(const AbcA::ObjectReader *this_);

AbcA::ArchiveReader*
Alembic_AbcCoreAbstract_v11_ObjectReader_getArchive(AbcA::ObjectReader *this_,
                                AbcA::ArchiveReaderPtr *oArchivePtr);

AbcA::ObjectReader*
Alembic_AbcCoreAbstract_v11_ObjectReader_getParent(AbcA::ObjectReader *this_,
                               AbcA::ObjectReaderPtr *oParent);

AbcA::CompoundPropertyReader*
Alembic_AbcCoreAbstract_v11_ObjectReader_getProperties(AbcA::ObjectReader *this_,
                                   AbcA::CompoundPropertyReaderPtr *oProperties);

size_t
Alembic_AbcCoreAbstract_v11_ObjectReader_getNumChildren(AbcA::ObjectReader *this_);

const AbcA::ObjectHeader *
Alembic_AbcCoreAbstract_v11_ObjectReader_getChildHeaderByIndex(AbcA::ObjectReader *this_, size_t i);

const AbcA::ObjectHeader *
Alembic_AbcCoreAbstract_v11_ObjectReader_getChildHeaderByName(AbcA::ObjectReader *this_, CStringRef iName);

AbcA::ObjectReader*
Alembic_AbcCoreAbstract_v11_ObjectReader_getChildByIndex(AbcA::ObjectReader *this_, size_t i,
                                                      AbcA::ObjectReaderPtr *oChild);

AbcA::ObjectReader*
Alembic_AbcCoreAbstract_v11_ObjectReader_getChildByName(AbcA::ObjectReader *this_,
                                                        CStringRef iName, AbcA::ObjectReaderPtr *oChild);

CStringRef
Alembic_AbcCoreAbstract_v11_ObjectHeader_getName(const AbcA::ObjectHeader *this_);

CStringRef
Alembic_AbcCoreAbstract_v11_PropertyHeader_getName(const AbcA::PropertyHeader *this_);

AbcA::PropertyType
Alembic_AbcCoreAbstract_v11_PropertyHeader_getPropertyType(const AbcA::PropertyHeader *this_);

const AbcA::PropertyHeader *
Alembic_AbcCoreAbstract_v11_BasePropertyReader_getHeader(const AbcA::BasePropertyReader *this_);

CStringRef
Alembic_AbcCoreAbstract_v11_BasePropertyReader_getName(
        const AbcA::BasePropertyReader *this_);

AbcA::ScalarPropertyReader*
Alembic_AbcCoreAbstract_v11_BasePropertyReader_asScalarPtr(AbcA::BasePropertyReader *this_, AbcA::ScalarPropertyReaderPtr *oProperty);

AbcA::ArrayPropertyReader*
Alembic_AbcCoreAbstract_v11_BasePropertyReader_asArrayPtr(AbcA::BasePropertyReader *this_, AbcA::ArrayPropertyReaderPtr *oProperty);

AbcA::CompoundPropertyReader*
Alembic_AbcCoreAbstract_v11_BasePropertyReader_asCompoundPtr(AbcA::BasePropertyReader *this_, AbcA::CompoundPropertyReaderPtr *oProperty);

size_t
Alembic_AbcCoreAbstract_v11_CompoundPropertyReader_getNumProperties(AbcA::CompoundPropertyReader *this_);

const AbcA::PropertyHeader *
Alembic_AbcCoreAbstract_v11_CompoundPropertyReader_getPropertyHeaderByIndex(AbcA::CompoundPropertyReader *this_, size_t index);

const AbcA::PropertyHeader *
Alembic_AbcCoreAbstract_v11_CompoundPropertyReader_getPropertyHeaderByName(AbcA::CompoundPropertyReader *this_, CStringRef iName);

//----------------------------------------------------------------------------------------------------------------------

void
Alembic_AbcCoreFactory_v11_IFactory_new(AbcF::IFactory *oThis_);

AbcA::ArchiveReader *
Alembic_AbcCoreFactory_v11_IFactory_getArchive(AbcF::IFactory *this_,
                                               CStringRef iFileName,
                                               AbcF::IFactory::CoreType *oType,
                                               AbcA::ArchiveReaderPtr *oArchivePtr);

void
Alembic_AbcCoreFactory_v11_IFactory_dtor(AbcF::IFactory* this_);

//----------------------------------------------------------------------------------------------------------------------

SliceU64 Alembic_AbcCoreAbstract_v11_ArraySample_dimensions(const AbcA::ArraySample* this_);
const AbcA::DataType* Alembic_AbcCoreAbstract_v11_ArraySample_getDataType(const AbcA::ArraySample* this_);

const void* Alembic_AbcCoreAbstract_v11_ArraySample_getData(const AbcA::ArraySample* this_);

}

