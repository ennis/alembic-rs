#include "glue.hpp"

namespace {
    template<typename T>
    void place(T *place, T init) {
        // place should be uninitialized
        new(place) T{std::move(init)};
    }
}

CStringRef Util_getStdString(const std::string* string)
{
    CStringRef out;
    out.ptr = string->data();
    out.len = string->size();
    return out;
}

AbcA::ArchiveReader *Alembic_AbcCoreAbstract_v11_ArchiveReaderPtr_get(AbcA::ArchiveReaderPtr *thisPtr_) {
    return (*thisPtr_).get();
}

const AbcA::ArchiveReader *Alembic_AbcCoreAbstract_v11_ArchiveReaderPtr_get_const(const AbcA::ArchiveReaderPtr *thisPtr_) {
    return (*thisPtr_).get();
}

CStringRef Alembic_AbcCoreAbstract_v11_ArchiveReader_getName(AbcA::ArchiveReader *this_) {
    return Util_getStdString(&this_->getName());
}

AbcA::ObjectReader* Alembic_AbcCoreAbstract_v11_ArchiveReader_getTop(AbcA::ArchiveReader *this_, AbcA::ObjectReaderPtr *oObjectReaderPtr) {
    new(oObjectReaderPtr) AbcA::ObjectReaderPtr{this_->getTop()};
    return oObjectReaderPtr->get();
}

void Alembic_AbcCoreAbstract_v11_ArchiveReaderPtr_dtor(AbcA::ArchiveReaderPtr* thisPtr_) {
    thisPtr_->~shared_ptr();
}

void Alembic_AbcCoreFactory_v11_IFactory_new(AbcF::IFactory *oThis_) {
    new(oThis_) AbcF::IFactory{};
}

AbcA::ArchiveReader* Alembic_AbcCoreFactory_v11_IFactory_getArchive(AbcF::IFactory *this_,
                                                    CStringRef iName,
                                 AbcF::IFactory::CoreType *oType,
                                 AbcA::ArchiveReaderPtr *oArchivePtr) {
    new(oArchivePtr) AbcA::ArchiveReaderPtr{this_->getArchive(std::string{ iName.ptr, iName.len }, *oType).getPtr()};
    return oArchivePtr->get();
}

void Alembic_AbcCoreFactory_v11_IFactory_dtor(AbcF::IFactory* this_) {
    this_->~IFactory();
}

const AbcA::ObjectHeader *Alembic_AbcCoreAbstract_v11_ObjectReader_getHeader(const AbcA::ObjectReader *this_) {
    return &this_->getHeader();
}

CStringRef Alembic_AbcCoreAbstract_v11_ObjectReader_getName(const AbcA::ObjectReader *this_) {
    return Util_getStdString(&this_->getName());
}

const AbcA::MetaData *Alembic_AbcCoreAbstract_v11_ObjectReader_getMetaData(const AbcA::ObjectReader *this_) {
    return &this_->getMetaData();
}

AbcA::ArchiveReader* Alembic_AbcCoreAbstract_v11_ObjectReader_getArchive(AbcA::ObjectReader *this_, AbcA::ArchiveReaderPtr *oArchivePtr) {
    new(oArchivePtr) AbcA::ArchiveReaderPtr{this_->getArchive()};
    return oArchivePtr->get();
}

AbcA::ObjectReader * Alembic_AbcCoreAbstract_v11_ObjectReader_getParent(AbcA::ObjectReader *this_, AbcA::ObjectReaderPtr *oParent) {
    new(oParent) AbcA::ObjectReaderPtr{this_->getParent()};
    return oParent->get();
}

AbcA::CompoundPropertyReader* Alembic_AbcCoreAbstract_v11_ObjectReader_getProperties(AbcA::ObjectReader *this_, AbcA::CompoundPropertyReaderPtr *oProperties) {
    new(oProperties) AbcA::CompoundPropertyReaderPtr{this_->getProperties()};
    return oProperties->get();
}

size_t Alembic_AbcCoreAbstract_v11_ObjectReader_getNumChildren(AbcA::ObjectReader *this_) {
    return this_->getNumChildren();
}

const AbcA::ObjectHeader *Alembic_AbcCoreAbstract_v11_ObjectReader_getChildHeaderByIndex(AbcA::ObjectReader *this_,
                                                                                      size_t i) {
    return &this_->getChildHeader(i);
}

const AbcA::ObjectHeader *Alembic_AbcCoreAbstract_v11_ObjectReader_getChildHeaderByName(AbcA::ObjectReader *this_, CStringRef iName) {
    return this_->getChildHeader(std::string{ iName.ptr, iName.len });
}

AbcA::ObjectReader * Alembic_AbcCoreAbstract_v11_ObjectReader_getChildByIndex(AbcA::ObjectReader *this_, size_t i,
                                                           AbcA::ObjectReaderPtr *oChild) {
    new(oChild) AbcA::ObjectReaderPtr{this_->getChild(i)};
    return oChild->get();
}

AbcA::ObjectReader * Alembic_AbcCoreAbstract_v11_ObjectReader_getChildByName(AbcA::ObjectReader *this_, CStringRef iName, AbcA::ObjectReaderPtr *oChild) {
    new(oChild) AbcA::ObjectReaderPtr{this_->getChild(std::string{ iName.ptr, iName.len })};
    return oChild->get();
}

CStringRef Alembic_AbcCoreAbstract_v11_ObjectHeader_getName(const AbcA::ObjectHeader *this_) {
    return Util_getStdString(&this_->getName());
}

const AbcA::PropertyHeader *Alembic_AbcCoreAbstract_v11_BasePropertyReader_getHeader(const AbcA::BasePropertyReader *this_) {
    return &this_->getHeader();
}

CStringRef Alembic_AbcCoreAbstract_v11_BasePropertyReader_getName(const AbcA::BasePropertyReader *this_) {
    return Util_getStdString(&this_->getName());
}

SliceU64 Alembic_AbcCoreAbstract_v11_ArraySample_dimensions(const AbcA::ArraySample *this_) {
    SliceU64 r;
    r.ptr = this_->getDimensions().rootPtr();
    r.len = this_->getDimensions().rank();
    return r;
}

const AbcA::DataType *Alembic_AbcCoreAbstract_v11_ArraySample_getDataType(const AbcA::ArraySample *this_) {
    return &this_->getDataType();
}

const void *Alembic_AbcCoreAbstract_v11_ArraySample_getData(const AbcA::ArraySample *this_) {
    return this_->getData();
}

const AbcA::PropertyHeader *
Alembic_AbcCoreAbstract_v11_CompoundPropertyReader_getPropertyHeaderByName(AbcA::CompoundPropertyReader *this_,
                                                                           CStringRef iName) {
    return this_->getPropertyHeader(std::string{iName.ptr, iName.len});
}

const AbcA::PropertyHeader *
Alembic_AbcCoreAbstract_v11_CompoundPropertyReader_getPropertyHeaderByIndex(AbcA::CompoundPropertyReader *this_,
                                                                            size_t index) {
    return &this_->getPropertyHeader(index);
}

size_t Alembic_AbcCoreAbstract_v11_CompoundPropertyReader_getNumProperties(AbcA::CompoundPropertyReader *this_) {
    return this_->getNumProperties();
}

CStringRef Alembic_AbcCoreAbstract_v11_PropertyHeader_getName(const AbcA::PropertyHeader *this_) {
    return Util_getStdString(&this_->getName());
}

AbcA::ScalarPropertyReader *Alembic_AbcCoreAbstract_v11_BasePropertyReader_asScalarPtr(AbcA::BasePropertyReader *this_,
                                                                                       AbcA::ScalarPropertyReaderPtr *oProperty) {
    new(oProperty) AbcA::ScalarPropertyReaderPtr{this_->asScalarPtr()};
    return oProperty->get();
}

AbcA::ArrayPropertyReader *Alembic_AbcCoreAbstract_v11_BasePropertyReader_asArrayPtr(AbcA::BasePropertyReader *this_,
                                                                                     AbcA::ArrayPropertyReaderPtr *oProperty) {
    new(oProperty) AbcA::ArrayPropertyReaderPtr{this_->asArrayPtr()};
    return oProperty->get();
}

AbcA::CompoundPropertyReader *
Alembic_AbcCoreAbstract_v11_BasePropertyReader_asCompoundPtr(AbcA::BasePropertyReader *this_,
                                                             AbcA::CompoundPropertyReaderPtr *oProperty) {
    new(oProperty) AbcA::CompoundPropertyReaderPtr{this_->asCompoundPtr()};
    return oProperty->get();
}

AbcA::PropertyType Alembic_AbcCoreAbstract_v11_PropertyHeader_getPropertyType(const AbcA::PropertyHeader *this_) {
    return this_->getPropertyType();
}



