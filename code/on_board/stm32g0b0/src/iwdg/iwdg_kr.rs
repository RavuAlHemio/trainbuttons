#[doc = "Register `IWDG_KR` writer"]
pub type W = crate::W<IwdgKrSpec>;
#[doc = "Field `KEY` writer - Key value (write only, read 0x0000)"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Key value (write only, read 0x0000)"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<IwdgKrSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_kr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgKrSpec;
impl crate::RegisterSpec for IwdgKrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iwdg_kr::W`](W) writer structure"]
impl crate::Writable for IwdgKrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWDG_KR to value 0"]
impl crate::Resettable for IwdgKrSpec {
    const RESET_VALUE: u32 = 0;
}
