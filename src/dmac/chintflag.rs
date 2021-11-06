#[doc = "Register `CHINTFLAG` reader"]
pub struct R(crate::R<CHINTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHINTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHINTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHINTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHINTFLAG` writer"]
pub struct W(crate::W<CHINTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHINTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CHINTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHINTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TERR` reader - Channel Transfer Error"]
pub struct TERR_R(crate::FieldReader<bool, bool>);
impl TERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERR` writer - Channel Transfer Error"]
pub struct TERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `TCMPL` reader - Channel Transfer Complete"]
pub struct TCMPL_R(crate::FieldReader<bool, bool>);
impl TCMPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCMPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCMPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCMPL` writer - Channel Transfer Complete"]
pub struct TCMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCMPL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SUSP` reader - Channel Suspend"]
pub struct SUSP_R(crate::FieldReader<bool, bool>);
impl SUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP` writer - Channel Suspend"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Transfer Error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Transfer Complete"]
    #[inline(always)]
    pub fn tcmpl(&self) -> TCMPL_R {
        TCMPL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Transfer Error"]
    #[inline(always)]
    pub fn terr(&mut self) -> TERR_W {
        TERR_W { w: self }
    }
    #[doc = "Bit 1 - Channel Transfer Complete"]
    #[inline(always)]
    pub fn tcmpl(&mut self) -> TCMPL_W {
        TCMPL_W { w: self }
    }
    #[doc = "Bit 2 - Channel Suspend"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chintflag](index.html) module"]
pub struct CHINTFLAG_SPEC;
impl crate::RegisterSpec for CHINTFLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chintflag::R](R) reader structure"]
impl crate::Readable for CHINTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chintflag::W](W) writer structure"]
impl crate::Writable for CHINTFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHINTFLAG to value 0"]
impl crate::Resettable for CHINTFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
