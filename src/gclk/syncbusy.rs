#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SWRST` reader - Software Reset Synchroniation Busy bit"]
pub struct SWRST_R(crate::FieldReader<bool, bool>);
impl SWRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Generic Clock Generator Control 0 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL0_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 0x1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 0x2,
    #[doc = "3: Generic clock generator 2"]
    GCLK2 = 0x4,
    #[doc = "4: Generic clock generator 3"]
    GCLK3 = 0x8,
    #[doc = "5: Generic clock generator 4"]
    GCLK4 = 0x10,
    #[doc = "6: Generic clock generator 5"]
    GCLK5 = 0x20,
    #[doc = "7: Generic clock generator 6"]
    GCLK6 = 0x40,
    #[doc = "8: Generic clock generator 7"]
    GCLK7 = 0x80,
    #[doc = "9: Generic clock generator 8"]
    GCLK8 = 0x100,
}
impl From<GENCTRL0_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENCTRL0` reader - Generic Clock Generator Control 0 Synchronization Busy bits"]
pub struct GENCTRL0_R(crate::FieldReader<bool, GENCTRL0_A>);
impl GENCTRL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL0_A> {
        match self.bits {
            true => Some(GENCTRL0_A::GCLK0),
            true => Some(GENCTRL0_A::GCLK1),
            true => Some(GENCTRL0_A::GCLK2),
            true => Some(GENCTRL0_A::GCLK3),
            true => Some(GENCTRL0_A::GCLK4),
            true => Some(GENCTRL0_A::GCLK5),
            true => Some(GENCTRL0_A::GCLK6),
            true => Some(GENCTRL0_A::GCLK7),
            true => Some(GENCTRL0_A::GCLK8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        **self == GENCTRL0_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        **self == GENCTRL0_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        **self == GENCTRL0_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        **self == GENCTRL0_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        **self == GENCTRL0_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        **self == GENCTRL0_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        **self == GENCTRL0_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        **self == GENCTRL0_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        **self == GENCTRL0_A::GCLK8
    }
}
impl core::ops::Deref for GENCTRL0_R {
    type Target = crate::FieldReader<bool, GENCTRL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Generic Clock Generator Control 1 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL1_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 0x1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 0x2,
    #[doc = "3: Generic clock generator 2"]
    GCLK2 = 0x4,
    #[doc = "4: Generic clock generator 3"]
    GCLK3 = 0x8,
    #[doc = "5: Generic clock generator 4"]
    GCLK4 = 0x10,
    #[doc = "6: Generic clock generator 5"]
    GCLK5 = 0x20,
    #[doc = "7: Generic clock generator 6"]
    GCLK6 = 0x40,
    #[doc = "8: Generic clock generator 7"]
    GCLK7 = 0x80,
    #[doc = "9: Generic clock generator 8"]
    GCLK8 = 0x100,
}
impl From<GENCTRL1_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENCTRL1` reader - Generic Clock Generator Control 1 Synchronization Busy bits"]
pub struct GENCTRL1_R(crate::FieldReader<bool, GENCTRL1_A>);
impl GENCTRL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL1_A> {
        match self.bits {
            true => Some(GENCTRL1_A::GCLK0),
            true => Some(GENCTRL1_A::GCLK1),
            true => Some(GENCTRL1_A::GCLK2),
            true => Some(GENCTRL1_A::GCLK3),
            true => Some(GENCTRL1_A::GCLK4),
            true => Some(GENCTRL1_A::GCLK5),
            true => Some(GENCTRL1_A::GCLK6),
            true => Some(GENCTRL1_A::GCLK7),
            true => Some(GENCTRL1_A::GCLK8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        **self == GENCTRL1_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        **self == GENCTRL1_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        **self == GENCTRL1_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        **self == GENCTRL1_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        **self == GENCTRL1_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        **self == GENCTRL1_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        **self == GENCTRL1_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        **self == GENCTRL1_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        **self == GENCTRL1_A::GCLK8
    }
}
impl core::ops::Deref for GENCTRL1_R {
    type Target = crate::FieldReader<bool, GENCTRL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Generic Clock Generator Control 2 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL2_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 0x1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 0x2,
    #[doc = "3: Generic clock generator 2"]
    GCLK2 = 0x4,
    #[doc = "4: Generic clock generator 3"]
    GCLK3 = 0x8,
    #[doc = "5: Generic clock generator 4"]
    GCLK4 = 0x10,
    #[doc = "6: Generic clock generator 5"]
    GCLK5 = 0x20,
    #[doc = "7: Generic clock generator 6"]
    GCLK6 = 0x40,
    #[doc = "8: Generic clock generator 7"]
    GCLK7 = 0x80,
    #[doc = "9: Generic clock generator 8"]
    GCLK8 = 0x100,
}
impl From<GENCTRL2_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENCTRL2` reader - Generic Clock Generator Control 2 Synchronization Busy bits"]
pub struct GENCTRL2_R(crate::FieldReader<bool, GENCTRL2_A>);
impl GENCTRL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL2_A> {
        match self.bits {
            true => Some(GENCTRL2_A::GCLK0),
            true => Some(GENCTRL2_A::GCLK1),
            true => Some(GENCTRL2_A::GCLK2),
            true => Some(GENCTRL2_A::GCLK3),
            true => Some(GENCTRL2_A::GCLK4),
            true => Some(GENCTRL2_A::GCLK5),
            true => Some(GENCTRL2_A::GCLK6),
            true => Some(GENCTRL2_A::GCLK7),
            true => Some(GENCTRL2_A::GCLK8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        **self == GENCTRL2_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        **self == GENCTRL2_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        **self == GENCTRL2_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        **self == GENCTRL2_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        **self == GENCTRL2_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        **self == GENCTRL2_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        **self == GENCTRL2_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        **self == GENCTRL2_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        **self == GENCTRL2_A::GCLK8
    }
}
impl core::ops::Deref for GENCTRL2_R {
    type Target = crate::FieldReader<bool, GENCTRL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Generic Clock Generator Control 3 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL3_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 0x1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 0x2,
    #[doc = "3: Generic clock generator 2"]
    GCLK2 = 0x4,
    #[doc = "4: Generic clock generator 3"]
    GCLK3 = 0x8,
    #[doc = "5: Generic clock generator 4"]
    GCLK4 = 0x10,
    #[doc = "6: Generic clock generator 5"]
    GCLK5 = 0x20,
    #[doc = "7: Generic clock generator 6"]
    GCLK6 = 0x40,
    #[doc = "8: Generic clock generator 7"]
    GCLK7 = 0x80,
    #[doc = "9: Generic clock generator 8"]
    GCLK8 = 0x100,
}
impl From<GENCTRL3_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENCTRL3` reader - Generic Clock Generator Control 3 Synchronization Busy bits"]
pub struct GENCTRL3_R(crate::FieldReader<bool, GENCTRL3_A>);
impl GENCTRL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL3_A> {
        match self.bits {
            true => Some(GENCTRL3_A::GCLK0),
            true => Some(GENCTRL3_A::GCLK1),
            true => Some(GENCTRL3_A::GCLK2),
            true => Some(GENCTRL3_A::GCLK3),
            true => Some(GENCTRL3_A::GCLK4),
            true => Some(GENCTRL3_A::GCLK5),
            true => Some(GENCTRL3_A::GCLK6),
            true => Some(GENCTRL3_A::GCLK7),
            true => Some(GENCTRL3_A::GCLK8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        **self == GENCTRL3_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        **self == GENCTRL3_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        **self == GENCTRL3_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        **self == GENCTRL3_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        **self == GENCTRL3_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        **self == GENCTRL3_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        **self == GENCTRL3_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        **self == GENCTRL3_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        **self == GENCTRL3_A::GCLK8
    }
}
impl core::ops::Deref for GENCTRL3_R {
    type Target = crate::FieldReader<bool, GENCTRL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Generic Clock Generator Control 4 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL4_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 0x1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 0x2,
    #[doc = "3: Generic clock generator 2"]
    GCLK2 = 0x4,
    #[doc = "4: Generic clock generator 3"]
    GCLK3 = 0x8,
    #[doc = "5: Generic clock generator 4"]
    GCLK4 = 0x10,
    #[doc = "6: Generic clock generator 5"]
    GCLK5 = 0x20,
    #[doc = "7: Generic clock generator 6"]
    GCLK6 = 0x40,
    #[doc = "8: Generic clock generator 7"]
    GCLK7 = 0x80,
    #[doc = "9: Generic clock generator 8"]
    GCLK8 = 0x100,
}
impl From<GENCTRL4_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENCTRL4` reader - Generic Clock Generator Control 4 Synchronization Busy bits"]
pub struct GENCTRL4_R(crate::FieldReader<bool, GENCTRL4_A>);
impl GENCTRL4_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL4_A> {
        match self.bits {
            true => Some(GENCTRL4_A::GCLK0),
            true => Some(GENCTRL4_A::GCLK1),
            true => Some(GENCTRL4_A::GCLK2),
            true => Some(GENCTRL4_A::GCLK3),
            true => Some(GENCTRL4_A::GCLK4),
            true => Some(GENCTRL4_A::GCLK5),
            true => Some(GENCTRL4_A::GCLK6),
            true => Some(GENCTRL4_A::GCLK7),
            true => Some(GENCTRL4_A::GCLK8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        **self == GENCTRL4_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        **self == GENCTRL4_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        **self == GENCTRL4_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        **self == GENCTRL4_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        **self == GENCTRL4_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        **self == GENCTRL4_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        **self == GENCTRL4_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        **self == GENCTRL4_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        **self == GENCTRL4_A::GCLK8
    }
}
impl core::ops::Deref for GENCTRL4_R {
    type Target = crate::FieldReader<bool, GENCTRL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Generic Clock Generator Control 5 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL5_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 0x1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 0x2,
    #[doc = "3: Generic clock generator 2"]
    GCLK2 = 0x4,
    #[doc = "4: Generic clock generator 3"]
    GCLK3 = 0x8,
    #[doc = "5: Generic clock generator 4"]
    GCLK4 = 0x10,
    #[doc = "6: Generic clock generator 5"]
    GCLK5 = 0x20,
    #[doc = "7: Generic clock generator 6"]
    GCLK6 = 0x40,
    #[doc = "8: Generic clock generator 7"]
    GCLK7 = 0x80,
    #[doc = "9: Generic clock generator 8"]
    GCLK8 = 0x100,
}
impl From<GENCTRL5_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENCTRL5` reader - Generic Clock Generator Control 5 Synchronization Busy bits"]
pub struct GENCTRL5_R(crate::FieldReader<bool, GENCTRL5_A>);
impl GENCTRL5_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL5_A> {
        match self.bits {
            true => Some(GENCTRL5_A::GCLK0),
            true => Some(GENCTRL5_A::GCLK1),
            true => Some(GENCTRL5_A::GCLK2),
            true => Some(GENCTRL5_A::GCLK3),
            true => Some(GENCTRL5_A::GCLK4),
            true => Some(GENCTRL5_A::GCLK5),
            true => Some(GENCTRL5_A::GCLK6),
            true => Some(GENCTRL5_A::GCLK7),
            true => Some(GENCTRL5_A::GCLK8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        **self == GENCTRL5_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        **self == GENCTRL5_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        **self == GENCTRL5_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        **self == GENCTRL5_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        **self == GENCTRL5_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        **self == GENCTRL5_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        **self == GENCTRL5_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        **self == GENCTRL5_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        **self == GENCTRL5_A::GCLK8
    }
}
impl core::ops::Deref for GENCTRL5_R {
    type Target = crate::FieldReader<bool, GENCTRL5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Generic Clock Generator Control 6 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL6_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 0x1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 0x2,
    #[doc = "3: Generic clock generator 2"]
    GCLK2 = 0x4,
    #[doc = "4: Generic clock generator 3"]
    GCLK3 = 0x8,
    #[doc = "5: Generic clock generator 4"]
    GCLK4 = 0x10,
    #[doc = "6: Generic clock generator 5"]
    GCLK5 = 0x20,
    #[doc = "7: Generic clock generator 6"]
    GCLK6 = 0x40,
    #[doc = "8: Generic clock generator 7"]
    GCLK7 = 0x80,
    #[doc = "9: Generic clock generator 8"]
    GCLK8 = 0x100,
}
impl From<GENCTRL6_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENCTRL6` reader - Generic Clock Generator Control 6 Synchronization Busy bits"]
pub struct GENCTRL6_R(crate::FieldReader<bool, GENCTRL6_A>);
impl GENCTRL6_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL6_A> {
        match self.bits {
            true => Some(GENCTRL6_A::GCLK0),
            true => Some(GENCTRL6_A::GCLK1),
            true => Some(GENCTRL6_A::GCLK2),
            true => Some(GENCTRL6_A::GCLK3),
            true => Some(GENCTRL6_A::GCLK4),
            true => Some(GENCTRL6_A::GCLK5),
            true => Some(GENCTRL6_A::GCLK6),
            true => Some(GENCTRL6_A::GCLK7),
            true => Some(GENCTRL6_A::GCLK8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        **self == GENCTRL6_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        **self == GENCTRL6_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        **self == GENCTRL6_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        **self == GENCTRL6_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        **self == GENCTRL6_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        **self == GENCTRL6_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        **self == GENCTRL6_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        **self == GENCTRL6_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        **self == GENCTRL6_A::GCLK8
    }
}
impl core::ops::Deref for GENCTRL6_R {
    type Target = crate::FieldReader<bool, GENCTRL6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Generic Clock Generator Control 7 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL7_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 0x1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 0x2,
    #[doc = "3: Generic clock generator 2"]
    GCLK2 = 0x4,
    #[doc = "4: Generic clock generator 3"]
    GCLK3 = 0x8,
    #[doc = "5: Generic clock generator 4"]
    GCLK4 = 0x10,
    #[doc = "6: Generic clock generator 5"]
    GCLK5 = 0x20,
    #[doc = "7: Generic clock generator 6"]
    GCLK6 = 0x40,
    #[doc = "8: Generic clock generator 7"]
    GCLK7 = 0x80,
    #[doc = "9: Generic clock generator 8"]
    GCLK8 = 0x100,
}
impl From<GENCTRL7_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENCTRL7` reader - Generic Clock Generator Control 7 Synchronization Busy bits"]
pub struct GENCTRL7_R(crate::FieldReader<bool, GENCTRL7_A>);
impl GENCTRL7_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL7_A> {
        match self.bits {
            true => Some(GENCTRL7_A::GCLK0),
            true => Some(GENCTRL7_A::GCLK1),
            true => Some(GENCTRL7_A::GCLK2),
            true => Some(GENCTRL7_A::GCLK3),
            true => Some(GENCTRL7_A::GCLK4),
            true => Some(GENCTRL7_A::GCLK5),
            true => Some(GENCTRL7_A::GCLK6),
            true => Some(GENCTRL7_A::GCLK7),
            true => Some(GENCTRL7_A::GCLK8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        **self == GENCTRL7_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        **self == GENCTRL7_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        **self == GENCTRL7_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        **self == GENCTRL7_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        **self == GENCTRL7_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        **self == GENCTRL7_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        **self == GENCTRL7_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        **self == GENCTRL7_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        **self == GENCTRL7_A::GCLK8
    }
}
impl core::ops::Deref for GENCTRL7_R {
    type Target = crate::FieldReader<bool, GENCTRL7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Generic Clock Generator Control 8 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL8_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 0x1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 0x2,
    #[doc = "3: Generic clock generator 2"]
    GCLK2 = 0x4,
    #[doc = "4: Generic clock generator 3"]
    GCLK3 = 0x8,
    #[doc = "5: Generic clock generator 4"]
    GCLK4 = 0x10,
    #[doc = "6: Generic clock generator 5"]
    GCLK5 = 0x20,
    #[doc = "7: Generic clock generator 6"]
    GCLK6 = 0x40,
    #[doc = "8: Generic clock generator 7"]
    GCLK7 = 0x80,
    #[doc = "9: Generic clock generator 8"]
    GCLK8 = 0x100,
}
impl From<GENCTRL8_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENCTRL8` reader - Generic Clock Generator Control 8 Synchronization Busy bits"]
pub struct GENCTRL8_R(crate::FieldReader<bool, GENCTRL8_A>);
impl GENCTRL8_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENCTRL8_A> {
        match self.bits {
            true => Some(GENCTRL8_A::GCLK0),
            true => Some(GENCTRL8_A::GCLK1),
            true => Some(GENCTRL8_A::GCLK2),
            true => Some(GENCTRL8_A::GCLK3),
            true => Some(GENCTRL8_A::GCLK4),
            true => Some(GENCTRL8_A::GCLK5),
            true => Some(GENCTRL8_A::GCLK6),
            true => Some(GENCTRL8_A::GCLK7),
            true => Some(GENCTRL8_A::GCLK8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        **self == GENCTRL8_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        **self == GENCTRL8_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        **self == GENCTRL8_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        **self == GENCTRL8_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        **self == GENCTRL8_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        **self == GENCTRL8_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        **self == GENCTRL8_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        **self == GENCTRL8_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        **self == GENCTRL8_A::GCLK8
    }
}
impl core::ops::Deref for GENCTRL8_R {
    type Target = crate::FieldReader<bool, GENCTRL8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Synchroniation Busy bit"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Generic Clock Generator Control 0 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl0(&self) -> GENCTRL0_R {
        GENCTRL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Generic Clock Generator Control 1 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl1(&self) -> GENCTRL1_R {
        GENCTRL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Generic Clock Generator Control 2 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl2(&self) -> GENCTRL2_R {
        GENCTRL2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Generic Clock Generator Control 3 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl3(&self) -> GENCTRL3_R {
        GENCTRL3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Generic Clock Generator Control 4 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl4(&self) -> GENCTRL4_R {
        GENCTRL4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Generic Clock Generator Control 5 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl5(&self) -> GENCTRL5_R {
        GENCTRL5_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generic Clock Generator Control 6 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl6(&self) -> GENCTRL6_R {
        GENCTRL6_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Generic Clock Generator Control 7 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl7(&self) -> GENCTRL7_R {
        GENCTRL7_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Generic Clock Generator Control 8 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl8(&self) -> GENCTRL8_R {
        GENCTRL8_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
