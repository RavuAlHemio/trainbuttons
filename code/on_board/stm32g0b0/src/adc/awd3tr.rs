#[doc = "Register `AWD3TR` reader"]
pub type R = crate::R<Awd3trSpec>;
#[doc = "Register `AWD3TR` writer"]
pub type W = crate::W<Awd3trSpec>;
#[doc = "Field `LT3` reader - Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on page355."]
pub type Lt3R = crate::FieldReader<u16>;
#[doc = "Field `LT3` writer - Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on page355."]
pub type Lt3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HT3` reader - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on page355."]
pub type Ht3R = crate::FieldReader<u16>;
#[doc = "Field `HT3` writer - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on page355."]
pub type Ht3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on page355."]
    #[inline(always)]
    pub fn lt3(&self) -> Lt3R {
        Lt3R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on page355."]
    #[inline(always)]
    pub fn ht3(&self) -> Ht3R {
        Ht3R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on page355."]
    #[inline(always)]
    #[must_use]
    pub fn lt3(&mut self) -> Lt3W<Awd3trSpec> {
        Lt3W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on page355."]
    #[inline(always)]
    #[must_use]
    pub fn ht3(&mut self) -> Ht3W<Awd3trSpec> {
        Ht3W::new(self, 16)
    }
}
#[doc = "ADC watchdog threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd3tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awd3trSpec;
impl crate::RegisterSpec for Awd3trSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd3tr::R`](R) reader structure"]
impl crate::Readable for Awd3trSpec {}
#[doc = "`write(|w| ..)` method takes [`awd3tr::W`](W) writer structure"]
impl crate::Writable for Awd3trSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWD3TR to value 0x0fff_0000"]
impl crate::Resettable for Awd3trSpec {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
