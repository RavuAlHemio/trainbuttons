#[doc = "Register `TAMP_SCR` writer"]
pub type W = crate::W<TampScrSpec>;
#[doc = "Field `CTAMP1F` writer - Clear TAMP1 detection flag Writing 1 in this bit clears the TAMP1F bit in the TAMP_SR register."]
pub type Ctamp1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP2F` writer - Clear TAMP2 detection flag Writing 1 in this bit clears the TAMP2F bit in the TAMP_SR register."]
pub type Ctamp2fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP3F` writer - Clear TAMP3 detection flag Writing 1 in this bit clears the TAMP3F bit in the TAMP_SR register."]
pub type Ctamp3fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP3F` writer - Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register."]
pub type Citamp3fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP4F` writer - Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register."]
pub type Citamp4fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP5F` writer - Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register."]
pub type Citamp5fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP6F` writer - Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register."]
pub type Citamp6fW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear TAMP1 detection flag Writing 1 in this bit clears the TAMP1F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp1f(&mut self) -> Ctamp1fW<TampScrSpec> {
        Ctamp1fW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear TAMP2 detection flag Writing 1 in this bit clears the TAMP2F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp2f(&mut self) -> Ctamp2fW<TampScrSpec> {
        Ctamp2fW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear TAMP3 detection flag Writing 1 in this bit clears the TAMP3F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp3f(&mut self) -> Ctamp3fW<TampScrSpec> {
        Ctamp3fW::new(self, 2)
    }
    #[doc = "Bit 18 - Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp3f(&mut self) -> Citamp3fW<TampScrSpec> {
        Citamp3fW::new(self, 18)
    }
    #[doc = "Bit 19 - Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp4f(&mut self) -> Citamp4fW<TampScrSpec> {
        Citamp4fW::new(self, 19)
    }
    #[doc = "Bit 20 - Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp5f(&mut self) -> Citamp5fW<TampScrSpec> {
        Citamp5fW::new(self, 20)
    }
    #[doc = "Bit 21 - Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp6f(&mut self) -> Citamp6fW<TampScrSpec> {
        Citamp6fW::new(self, 21)
    }
}
#[doc = "TAMP status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampScrSpec;
impl crate::RegisterSpec for TampScrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tamp_scr::W`](W) writer structure"]
impl crate::Writable for TampScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_SCR to value 0"]
impl crate::Resettable for TampScrSpec {
    const RESET_VALUE: u32 = 0;
}
