#[doc = "Register `USB_CNTR` reader"]
pub type R = crate::R<UsbCntrSpec>;
#[doc = "Register `USB_CNTR` writer"]
pub type W = crate::W<UsbCntrSpec>;
#[doc = "USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrst {
    #[doc = "0: No effect"]
    B0x0DeviceMode = 0,
    #[doc = "1: USB core is under reset"]
    B0x1DeviceMode = 1,
}
impl From<Usbrst> for bool {
    #[inline(always)]
    fn from(variant: Usbrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRST` reader - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
pub type UsbrstR = crate::BitReader<Usbrst>;
impl UsbrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbrst {
        match self.bits {
            false => Usbrst::B0x0DeviceMode,
            true => Usbrst::B0x1DeviceMode,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0_device_mode(&self) -> bool {
        *self == Usbrst::B0x0DeviceMode
    }
    #[doc = "USB core is under reset"]
    #[inline(always)]
    pub fn is_b_0x1_device_mode(&self) -> bool {
        *self == Usbrst::B0x1DeviceMode
    }
}
#[doc = "Field `USBRST` writer - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG, Usbrst>;
impl<'a, REG> UsbrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0_device_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrst::B0x0DeviceMode)
    }
    #[doc = "USB core is under reset"]
    #[inline(always)]
    pub fn b_0x1_device_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrst::B0x1DeviceMode)
    }
}
#[doc = "Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdwn {
    #[doc = "0: Exit power down."]
    B0x0 = 0,
    #[doc = "1: Enter power down mode."]
    B0x1 = 1,
}
impl From<Pdwn> for bool {
    #[inline(always)]
    fn from(variant: Pdwn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDWN` reader - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
pub type PdwnR = crate::BitReader<Pdwn>;
impl PdwnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdwn {
        match self.bits {
            false => Pdwn::B0x0,
            true => Pdwn::B0x1,
        }
    }
    #[doc = "Exit power down."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pdwn::B0x0
    }
    #[doc = "Enter power down mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pdwn::B0x1
    }
}
#[doc = "Field `PDWN` writer - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
pub type PdwnW<'a, REG> = crate::BitWriter<'a, REG, Pdwn>;
impl<'a, REG> PdwnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exit power down."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdwn::B0x0)
    }
    #[doc = "Enter power down mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdwn::B0x1)
    }
}
#[doc = "Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Susprdy {
    #[doc = "0: Normal operation"]
    B0x0 = 0,
    #[doc = "1: Suspend state"]
    B0x1 = 1,
}
impl From<Susprdy> for bool {
    #[inline(always)]
    fn from(variant: Susprdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPRDY` reader - Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set."]
pub type SusprdyR = crate::BitReader<Susprdy>;
impl SusprdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Susprdy {
        match self.bits {
            false => Susprdy::B0x0,
            true => Susprdy::B0x1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Susprdy::B0x0
    }
    #[doc = "Suspend state"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Susprdy::B0x1
    }
}
#[doc = "Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspen {
    #[doc = "0: No effect."]
    B0x0DeviceMode = 0,
    #[doc = "1: Enter L1/L2 suspend"]
    B0x1DeviceMode = 1,
}
impl From<Suspen> for bool {
    #[inline(always)]
    fn from(variant: Suspen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPEN` reader - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
pub type SuspenR = crate::BitReader<Suspen>;
impl SuspenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspen {
        match self.bits {
            false => Suspen::B0x0DeviceMode,
            true => Suspen::B0x1DeviceMode,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_b_0x0_device_mode(&self) -> bool {
        *self == Suspen::B0x0DeviceMode
    }
    #[doc = "Enter L1/L2 suspend"]
    #[inline(always)]
    pub fn is_b_0x1_device_mode(&self) -> bool {
        *self == Suspen::B0x1DeviceMode
    }
}
#[doc = "Field `SUSPEN` writer - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
pub type SuspenW<'a, REG> = crate::BitWriter<'a, REG, Suspen>;
impl<'a, REG> SuspenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn b_0x0_device_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Suspen::B0x0DeviceMode)
    }
    #[doc = "Enter L1/L2 suspend"]
    #[inline(always)]
    pub fn b_0x1_device_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Suspen::B0x1DeviceMode)
    }
}
#[doc = "L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2res {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Send L2 resume signaling to device"]
    B0x1 = 1,
}
impl From<L2res> for bool {
    #[inline(always)]
    fn from(variant: L2res) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L2RES` reader - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
pub type L2resR = crate::BitReader<L2res>;
impl L2resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L2res {
        match self.bits {
            false => L2res::B0x0,
            true => L2res::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == L2res::B0x0
    }
    #[doc = "Send L2 resume signaling to device"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == L2res::B0x1
    }
}
#[doc = "Field `L2RES` writer - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
pub type L2resW<'a, REG> = crate::BitWriter<'a, REG, L2res>;
impl<'a, REG> L2resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(L2res::B0x0)
    }
    #[doc = "Send L2 resume signaling to device"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(L2res::B0x1)
    }
}
#[doc = "L1 remote wakeup / resume driver Device mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1res {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Send 50 remote-wakeup signaling to host"]
    B0x1 = 1,
}
impl From<L1res> for bool {
    #[inline(always)]
    fn from(variant: L1res) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1RES` reader - L1 remote wakeup / resume driver Device mode"]
pub type L1resR = crate::BitReader<L1res>;
impl L1resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L1res {
        match self.bits {
            false => L1res::B0x0,
            true => L1res::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == L1res::B0x0
    }
    #[doc = "Send 50 remote-wakeup signaling to host"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == L1res::B0x1
    }
}
#[doc = "Field `L1RES` writer - L1 remote wakeup / resume driver Device mode"]
pub type L1resW<'a, REG> = crate::BitWriter<'a, REG, L1res>;
impl<'a, REG> L1resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(L1res::B0x0)
    }
    #[doc = "Send 50 remote-wakeup signaling to host"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(L1res::B0x1)
    }
}
#[doc = "LPM L1 state request interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1reqm {
    #[doc = "0: LPM L1 state request (L1REQ) interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: L1REQ interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B0x1 = 1,
}
impl From<L1reqm> for bool {
    #[inline(always)]
    fn from(variant: L1reqm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1REQM` reader - LPM L1 state request interrupt mask"]
pub type L1reqmR = crate::BitReader<L1reqm>;
impl L1reqmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L1reqm {
        match self.bits {
            false => L1reqm::B0x0,
            true => L1reqm::B0x1,
        }
    }
    #[doc = "LPM L1 state request (L1REQ) interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == L1reqm::B0x0
    }
    #[doc = "L1REQ interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == L1reqm::B0x1
    }
}
#[doc = "Field `L1REQM` writer - LPM L1 state request interrupt mask"]
pub type L1reqmW<'a, REG> = crate::BitWriter<'a, REG, L1reqm>;
impl<'a, REG> L1reqmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPM L1 state request (L1REQ) interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(L1reqm::B0x0)
    }
    #[doc = "L1REQ interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(L1reqm::B0x1)
    }
}
#[doc = "Expected start of frame interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esofm {
    #[doc = "0: Expected start of frame (ESOF) interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: ESOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B0x1 = 1,
}
impl From<Esofm> for bool {
    #[inline(always)]
    fn from(variant: Esofm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESOFM` reader - Expected start of frame interrupt mask"]
pub type EsofmR = crate::BitReader<Esofm>;
impl EsofmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Esofm {
        match self.bits {
            false => Esofm::B0x0,
            true => Esofm::B0x1,
        }
    }
    #[doc = "Expected start of frame (ESOF) interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Esofm::B0x0
    }
    #[doc = "ESOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Esofm::B0x1
    }
}
#[doc = "Field `ESOFM` writer - Expected start of frame interrupt mask"]
pub type EsofmW<'a, REG> = crate::BitWriter<'a, REG, Esofm>;
impl<'a, REG> EsofmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Expected start of frame (ESOF) interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Esofm::B0x0)
    }
    #[doc = "ESOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Esofm::B0x1)
    }
}
#[doc = "Start of frame interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sofm {
    #[doc = "0: SOF interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: SOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B0x1 = 1,
}
impl From<Sofm> for bool {
    #[inline(always)]
    fn from(variant: Sofm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFM` reader - Start of frame interrupt mask"]
pub type SofmR = crate::BitReader<Sofm>;
impl SofmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sofm {
        match self.bits {
            false => Sofm::B0x0,
            true => Sofm::B0x1,
        }
    }
    #[doc = "SOF interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sofm::B0x0
    }
    #[doc = "SOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sofm::B0x1
    }
}
#[doc = "Field `SOFM` writer - Start of frame interrupt mask"]
pub type SofmW<'a, REG> = crate::BitWriter<'a, REG, Sofm>;
impl<'a, REG> SofmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SOF interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sofm::B0x0)
    }
    #[doc = "SOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sofm::B0x1)
    }
}
#[doc = "USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstDconm {
    #[doc = "0: RESET interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: RESET interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B0x1 = 1,
}
impl From<RstDconm> for bool {
    #[inline(always)]
    fn from(variant: RstDconm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST_DCONM` reader - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
pub type RstDconmR = crate::BitReader<RstDconm>;
impl RstDconmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstDconm {
        match self.bits {
            false => RstDconm::B0x0,
            true => RstDconm::B0x1,
        }
    }
    #[doc = "RESET interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RstDconm::B0x0
    }
    #[doc = "RESET interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RstDconm::B0x1
    }
}
#[doc = "Field `RST_DCONM` writer - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
pub type RstDconmW<'a, REG> = crate::BitWriter<'a, REG, RstDconm>;
impl<'a, REG> RstDconmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RESET interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RstDconm::B0x0)
    }
    #[doc = "RESET interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RstDconm::B0x1)
    }
}
#[doc = "Suspend mode interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspm {
    #[doc = "0: Suspend mode request (SUSP) interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: SUSP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B0x1 = 1,
}
impl From<Suspm> for bool {
    #[inline(always)]
    fn from(variant: Suspm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPM` reader - Suspend mode interrupt mask"]
pub type SuspmR = crate::BitReader<Suspm>;
impl SuspmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspm {
        match self.bits {
            false => Suspm::B0x0,
            true => Suspm::B0x1,
        }
    }
    #[doc = "Suspend mode request (SUSP) interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Suspm::B0x0
    }
    #[doc = "SUSP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Suspm::B0x1
    }
}
#[doc = "Field `SUSPM` writer - Suspend mode interrupt mask"]
pub type SuspmW<'a, REG> = crate::BitWriter<'a, REG, Suspm>;
impl<'a, REG> SuspmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Suspend mode request (SUSP) interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Suspm::B0x0)
    }
    #[doc = "SUSP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Suspm::B0x1)
    }
}
#[doc = "Wakeup interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupm {
    #[doc = "0: WKUP interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: WKUP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B0x1 = 1,
}
impl From<Wkupm> for bool {
    #[inline(always)]
    fn from(variant: Wkupm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPM` reader - Wakeup interrupt mask"]
pub type WkupmR = crate::BitReader<Wkupm>;
impl WkupmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupm {
        match self.bits {
            false => Wkupm::B0x0,
            true => Wkupm::B0x1,
        }
    }
    #[doc = "WKUP interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wkupm::B0x0
    }
    #[doc = "WKUP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wkupm::B0x1
    }
}
#[doc = "Field `WKUPM` writer - Wakeup interrupt mask"]
pub type WkupmW<'a, REG> = crate::BitWriter<'a, REG, Wkupm>;
impl<'a, REG> WkupmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WKUP interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupm::B0x0)
    }
    #[doc = "WKUP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupm::B0x1)
    }
}
#[doc = "Error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errm {
    #[doc = "0: ERR interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: ERR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B0x1 = 1,
}
impl From<Errm> for bool {
    #[inline(always)]
    fn from(variant: Errm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRM` reader - Error interrupt mask"]
pub type ErrmR = crate::BitReader<Errm>;
impl ErrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errm {
        match self.bits {
            false => Errm::B0x0,
            true => Errm::B0x1,
        }
    }
    #[doc = "ERR interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Errm::B0x0
    }
    #[doc = "ERR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Errm::B0x1
    }
}
#[doc = "Field `ERRM` writer - Error interrupt mask"]
pub type ErrmW<'a, REG> = crate::BitWriter<'a, REG, Errm>;
impl<'a, REG> ErrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERR interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Errm::B0x0)
    }
    #[doc = "ERR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Errm::B0x1)
    }
}
#[doc = "Packet memory area over / underrun interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmaovrm {
    #[doc = "0: PMAOVR interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: PMAOVR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B0x1 = 1,
}
impl From<Pmaovrm> for bool {
    #[inline(always)]
    fn from(variant: Pmaovrm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAOVRM` reader - Packet memory area over / underrun interrupt mask"]
pub type PmaovrmR = crate::BitReader<Pmaovrm>;
impl PmaovrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmaovrm {
        match self.bits {
            false => Pmaovrm::B0x0,
            true => Pmaovrm::B0x1,
        }
    }
    #[doc = "PMAOVR interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pmaovrm::B0x0
    }
    #[doc = "PMAOVR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pmaovrm::B0x1
    }
}
#[doc = "Field `PMAOVRM` writer - Packet memory area over / underrun interrupt mask"]
pub type PmaovrmW<'a, REG> = crate::BitWriter<'a, REG, Pmaovrm>;
impl<'a, REG> PmaovrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PMAOVR interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmaovrm::B0x0)
    }
    #[doc = "PMAOVR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmaovrm::B0x1)
    }
}
#[doc = "Correct transfer interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrm {
    #[doc = "0: Correct transfer (CTR) interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: CTR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B0x1 = 1,
}
impl From<Ctrm> for bool {
    #[inline(always)]
    fn from(variant: Ctrm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRM` reader - Correct transfer interrupt mask"]
pub type CtrmR = crate::BitReader<Ctrm>;
impl CtrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrm {
        match self.bits {
            false => Ctrm::B0x0,
            true => Ctrm::B0x1,
        }
    }
    #[doc = "Correct transfer (CTR) interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ctrm::B0x0
    }
    #[doc = "CTR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ctrm::B0x1
    }
}
#[doc = "Field `CTRM` writer - Correct transfer interrupt mask"]
pub type CtrmW<'a, REG> = crate::BitWriter<'a, REG, Ctrm>;
impl<'a, REG> CtrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Correct transfer (CTR) interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrm::B0x0)
    }
    #[doc = "CTR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrm::B0x1)
    }
}
#[doc = "512 byte threshold interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thr512m {
    #[doc = "0: 512 byte threshold interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: 512 byte threshold interrupt enabled"]
    B0x1 = 1,
}
impl From<Thr512m> for bool {
    #[inline(always)]
    fn from(variant: Thr512m) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THR512M` reader - 512 byte threshold interrupt mask"]
pub type Thr512mR = crate::BitReader<Thr512m>;
impl Thr512mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Thr512m {
        match self.bits {
            false => Thr512m::B0x0,
            true => Thr512m::B0x1,
        }
    }
    #[doc = "512 byte threshold interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Thr512m::B0x0
    }
    #[doc = "512 byte threshold interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Thr512m::B0x1
    }
}
#[doc = "Field `THR512M` writer - 512 byte threshold interrupt mask"]
pub type Thr512mW<'a, REG> = crate::BitWriter<'a, REG, Thr512m>;
impl<'a, REG> Thr512mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "512 byte threshold interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Thr512m::B0x0)
    }
    #[doc = "512 byte threshold interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Thr512m::B0x1)
    }
}
#[doc = "HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Host {
    #[doc = "0: USB Device function"]
    B0x0 = 0,
    #[doc = "1: USB host function"]
    B0x1 = 1,
}
impl From<Host> for bool {
    #[inline(always)]
    fn from(variant: Host) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST` reader - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
pub type HostR = crate::BitReader<Host>;
impl HostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Host {
        match self.bits {
            false => Host::B0x0,
            true => Host::B0x1,
        }
    }
    #[doc = "USB Device function"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Host::B0x0
    }
    #[doc = "USB host function"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Host::B0x1
    }
}
#[doc = "Field `HOST` writer - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
pub type HostW<'a, REG> = crate::BitWriter<'a, REG, Host>;
impl<'a, REG> HostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB Device function"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Host::B0x0)
    }
    #[doc = "USB host function"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Host::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
    #[inline(always)]
    pub fn pdwn(&self) -> PdwnR {
        PdwnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set."]
    #[inline(always)]
    pub fn susprdy(&self) -> SusprdyR {
        SusprdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
    #[inline(always)]
    pub fn suspen(&self) -> SuspenR {
        SuspenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
    #[inline(always)]
    pub fn l2res(&self) -> L2resR {
        L2resR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L1 remote wakeup / resume driver Device mode"]
    #[inline(always)]
    pub fn l1res(&self) -> L1resR {
        L1resR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    pub fn l1reqm(&self) -> L1reqmR {
        L1reqmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn esofm(&self) -> EsofmR {
        EsofmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SofmR {
        SofmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
    #[inline(always)]
    pub fn rst_dconm(&self) -> RstDconmR {
        RstDconmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn suspm(&self) -> SuspmR {
        SuspmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupm(&self) -> WkupmR {
        WkupmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errm(&self) -> ErrmR {
        ErrmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn pmaovrm(&self) -> PmaovrmR {
        PmaovrmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn ctrm(&self) -> CtrmR {
        CtrmR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 512 byte threshold interrupt mask"]
    #[inline(always)]
    pub fn thr512m(&self) -> Thr512mR {
        Thr512mR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
    #[inline(always)]
    pub fn host(&self) -> HostR {
        HostR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> UsbrstW<UsbCntrSpec> {
        UsbrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
    #[inline(always)]
    #[must_use]
    pub fn pdwn(&mut self) -> PdwnW<UsbCntrSpec> {
        PdwnW::new(self, 1)
    }
    #[doc = "Bit 3 - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
    #[inline(always)]
    #[must_use]
    pub fn suspen(&mut self) -> SuspenW<UsbCntrSpec> {
        SuspenW::new(self, 3)
    }
    #[doc = "Bit 4 - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn l2res(&mut self) -> L2resW<UsbCntrSpec> {
        L2resW::new(self, 4)
    }
    #[doc = "Bit 5 - L1 remote wakeup / resume driver Device mode"]
    #[inline(always)]
    #[must_use]
    pub fn l1res(&mut self) -> L1resW<UsbCntrSpec> {
        L1resW::new(self, 5)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn l1reqm(&mut self) -> L1reqmW<UsbCntrSpec> {
        L1reqmW::new(self, 7)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn esofm(&mut self) -> EsofmW<UsbCntrSpec> {
        EsofmW::new(self, 8)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SofmW<UsbCntrSpec> {
        SofmW::new(self, 9)
    }
    #[doc = "Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rst_dconm(&mut self) -> RstDconmW<UsbCntrSpec> {
        RstDconmW::new(self, 10)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn suspm(&mut self) -> SuspmW<UsbCntrSpec> {
        SuspmW::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn wkupm(&mut self) -> WkupmW<UsbCntrSpec> {
        WkupmW::new(self, 12)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn errm(&mut self) -> ErrmW<UsbCntrSpec> {
        ErrmW::new(self, 13)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn pmaovrm(&mut self) -> PmaovrmW<UsbCntrSpec> {
        PmaovrmW::new(self, 14)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ctrm(&mut self) -> CtrmW<UsbCntrSpec> {
        CtrmW::new(self, 15)
    }
    #[doc = "Bit 16 - 512 byte threshold interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn thr512m(&mut self) -> Thr512mW<UsbCntrSpec> {
        Thr512mW::new(self, 16)
    }
    #[doc = "Bit 31 - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn host(&mut self) -> HostW<UsbCntrSpec> {
        HostW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_cntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_cntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbCntrSpec;
impl crate::RegisterSpec for UsbCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_cntr::R`](R) reader structure"]
impl crate::Readable for UsbCntrSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_cntr::W`](W) writer structure"]
impl crate::Writable for UsbCntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB_CNTR to value 0x03"]
impl crate::Resettable for UsbCntrSpec {
    const RESET_VALUE: u32 = 0x03;
}
