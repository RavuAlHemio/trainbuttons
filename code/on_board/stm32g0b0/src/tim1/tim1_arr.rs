#[doc = "Register `TIM1_ARR` reader"]
pub type R = crate::R<Tim1ArrSpec>;
#[doc = "Register `TIM1_ARR` writer"]
pub type W = crate::W<Tim1ArrSpec>;
#[doc = "Field `ARR` reader - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null."]
pub type ArrR = crate::FieldReader<u16>;
#[doc = "Field `ARR` writer - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null."]
pub type ArrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null."]
    #[inline(always)]
    pub fn arr(&self) -> ArrR {
        ArrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null."]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ArrW<Tim1ArrSpec> {
        ArrW::new(self, 0)
    }
}
#[doc = "auto-reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim1_arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1ArrSpec;
impl crate::RegisterSpec for Tim1ArrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_arr::R`](R) reader structure"]
impl crate::Readable for Tim1ArrSpec {}
#[doc = "`write(|w| ..)` method takes [`tim1_arr::W`](W) writer structure"]
impl crate::Writable for Tim1ArrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_ARR to value 0xffff"]
impl crate::Resettable for Tim1ArrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
