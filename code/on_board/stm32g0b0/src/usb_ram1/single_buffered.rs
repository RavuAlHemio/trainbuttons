#[repr(C)]
#[doc = "Descriptors if double buffering is disabled."]
#[doc(alias = "SINGLE_BUFFERED")]
pub struct SingleBuffered {
    chep_txrxbd_0: ChepTxrxbd0,
    chep_rxtxbd_0: ChepRxtxbd0,
}
impl SingleBuffered {
    #[doc = "0x00 - Channel/endpoint transmit buffer descriptor 0"]
    #[inline(always)]
    pub const fn chep_txrxbd_0(&self) -> &ChepTxrxbd0 {
        &self.chep_txrxbd_0
    }
    #[doc = "0x04 - Channel/endpoint receive buffer descriptor 0"]
    #[inline(always)]
    pub const fn chep_rxtxbd_0(&self) -> &ChepRxtxbd0 {
        &self.chep_rxtxbd_0
    }
}
#[doc = "CHEP_TXRXBD_0 (rw) register accessor: Channel/endpoint transmit buffer descriptor 0\n\nYou can [`read`](crate::Reg::read) this register and get [`chep_txrxbd_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep_txrxbd_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep_txrxbd_0`]
module"]
#[doc(alias = "CHEP_TXRXBD_0")]
pub type ChepTxrxbd0 = crate::Reg<chep_txrxbd_0::ChepTxrxbd0Spec>;
#[doc = "Channel/endpoint transmit buffer descriptor 0"]
pub mod chep_txrxbd_0;
#[doc = "CHEP_RXTXBD_0 (rw) register accessor: Channel/endpoint receive buffer descriptor 0\n\nYou can [`read`](crate::Reg::read) this register and get [`chep_rxtxbd_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep_rxtxbd_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep_rxtxbd_0`]
module"]
#[doc(alias = "CHEP_RXTXBD_0")]
pub type ChepRxtxbd0 = crate::Reg<chep_rxtxbd_0::ChepRxtxbd0Spec>;
#[doc = "Channel/endpoint receive buffer descriptor 0"]
pub mod chep_rxtxbd_0;
