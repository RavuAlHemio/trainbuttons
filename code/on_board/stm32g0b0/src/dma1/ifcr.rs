#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IfcrSpec>;
#[doc = "Field `CGIF1` writer - global interrupt flag clear for channel 1"]
pub type Cgif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF1` writer - transfer complete flag clear for channel 1"]
pub type Ctcif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF1` writer - half transfer flag clear for channel 1"]
pub type Chtif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF1` writer - transfer error flag clear for channel 1"]
pub type Cteif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF2` writer - global interrupt flag clear for channel 2"]
pub type Cgif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF2` writer - transfer complete flag clear for channel 2"]
pub type Ctcif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF2` writer - half transfer flag clear for channel 2"]
pub type Chtif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF2` writer - transfer error flag clear for channel 2"]
pub type Cteif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF3` writer - global interrupt flag clear for channel 3"]
pub type Cgif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF3` writer - transfer complete flag clear for channel 3"]
pub type Ctcif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF3` writer - half transfer flag clear for channel 3"]
pub type Chtif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF3` writer - transfer error flag clear for channel 3"]
pub type Cteif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF4` writer - global interrupt flag clear for channel 4"]
pub type Cgif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF4` writer - transfer complete flag clear for channel 4"]
pub type Ctcif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF4` writer - half transfer flag clear for channel 4"]
pub type Chtif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF4` writer - transfer error flag clear for channel 4"]
pub type Cteif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF5` writer - global interrupt flag clear for channel 5"]
pub type Cgif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF5` writer - transfer complete flag clear for channel 5"]
pub type Ctcif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF5` writer - half transfer flag clear for channel 5"]
pub type Chtif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF5` writer - transfer error flag clear for channel 5"]
pub type Cteif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF6` writer - global interrupt flag clear for channel 6"]
pub type Cgif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF6` writer - transfer complete flag clear for channel 6"]
pub type Ctcif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF6` writer - half transfer flag clear for channel 6"]
pub type Chtif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF6` writer - transfer error flag clear for channel 6"]
pub type Cteif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF7` writer - global interrupt flag clear for channel 7"]
pub type Cgif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF7` writer - transfer complete flag clear for channel 7"]
pub type Ctcif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF7` writer - half transfer flag clear for channel 7"]
pub type Chtif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF7` writer - transfer error flag clear for channel 7"]
pub type Cteif7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - global interrupt flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cgif1(&mut self) -> Cgif1W<IfcrSpec> {
        Cgif1W::new(self, 0)
    }
    #[doc = "Bit 1 - transfer complete flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> Ctcif1W<IfcrSpec> {
        Ctcif1W::new(self, 1)
    }
    #[doc = "Bit 2 - half transfer flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> Chtif1W<IfcrSpec> {
        Chtif1W::new(self, 2)
    }
    #[doc = "Bit 3 - transfer error flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> Cteif1W<IfcrSpec> {
        Cteif1W::new(self, 3)
    }
    #[doc = "Bit 4 - global interrupt flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cgif2(&mut self) -> Cgif2W<IfcrSpec> {
        Cgif2W::new(self, 4)
    }
    #[doc = "Bit 5 - transfer complete flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> Ctcif2W<IfcrSpec> {
        Ctcif2W::new(self, 5)
    }
    #[doc = "Bit 6 - half transfer flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> Chtif2W<IfcrSpec> {
        Chtif2W::new(self, 6)
    }
    #[doc = "Bit 7 - transfer error flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> Cteif2W<IfcrSpec> {
        Cteif2W::new(self, 7)
    }
    #[doc = "Bit 8 - global interrupt flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cgif3(&mut self) -> Cgif3W<IfcrSpec> {
        Cgif3W::new(self, 8)
    }
    #[doc = "Bit 9 - transfer complete flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> Ctcif3W<IfcrSpec> {
        Ctcif3W::new(self, 9)
    }
    #[doc = "Bit 10 - half transfer flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> Chtif3W<IfcrSpec> {
        Chtif3W::new(self, 10)
    }
    #[doc = "Bit 11 - transfer error flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> Cteif3W<IfcrSpec> {
        Cteif3W::new(self, 11)
    }
    #[doc = "Bit 12 - global interrupt flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn cgif4(&mut self) -> Cgif4W<IfcrSpec> {
        Cgif4W::new(self, 12)
    }
    #[doc = "Bit 13 - transfer complete flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif4(&mut self) -> Ctcif4W<IfcrSpec> {
        Ctcif4W::new(self, 13)
    }
    #[doc = "Bit 14 - half transfer flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn chtif4(&mut self) -> Chtif4W<IfcrSpec> {
        Chtif4W::new(self, 14)
    }
    #[doc = "Bit 15 - transfer error flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn cteif4(&mut self) -> Cteif4W<IfcrSpec> {
        Cteif4W::new(self, 15)
    }
    #[doc = "Bit 16 - global interrupt flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn cgif5(&mut self) -> Cgif5W<IfcrSpec> {
        Cgif5W::new(self, 16)
    }
    #[doc = "Bit 17 - transfer complete flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif5(&mut self) -> Ctcif5W<IfcrSpec> {
        Ctcif5W::new(self, 17)
    }
    #[doc = "Bit 18 - half transfer flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn chtif5(&mut self) -> Chtif5W<IfcrSpec> {
        Chtif5W::new(self, 18)
    }
    #[doc = "Bit 19 - transfer error flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn cteif5(&mut self) -> Cteif5W<IfcrSpec> {
        Cteif5W::new(self, 19)
    }
    #[doc = "Bit 20 - global interrupt flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn cgif6(&mut self) -> Cgif6W<IfcrSpec> {
        Cgif6W::new(self, 20)
    }
    #[doc = "Bit 21 - transfer complete flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif6(&mut self) -> Ctcif6W<IfcrSpec> {
        Ctcif6W::new(self, 21)
    }
    #[doc = "Bit 22 - half transfer flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn chtif6(&mut self) -> Chtif6W<IfcrSpec> {
        Chtif6W::new(self, 22)
    }
    #[doc = "Bit 23 - transfer error flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn cteif6(&mut self) -> Cteif6W<IfcrSpec> {
        Cteif6W::new(self, 23)
    }
    #[doc = "Bit 24 - global interrupt flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn cgif7(&mut self) -> Cgif7W<IfcrSpec> {
        Cgif7W::new(self, 24)
    }
    #[doc = "Bit 25 - transfer complete flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif7(&mut self) -> Ctcif7W<IfcrSpec> {
        Ctcif7W::new(self, 25)
    }
    #[doc = "Bit 26 - half transfer flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn chtif7(&mut self) -> Chtif7W<IfcrSpec> {
        Chtif7W::new(self, 26)
    }
    #[doc = "Bit 27 - transfer error flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn cteif7(&mut self) -> Cteif7W<IfcrSpec> {
        Cteif7W::new(self, 27)
    }
}
#[doc = "DMA interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcrSpec;
impl crate::RegisterSpec for IfcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IfcrSpec {
    const RESET_VALUE: u32 = 0;
}
