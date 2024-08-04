#[repr(C)]
#[doc = "Descriptors if double buffering is enabled."]
#[doc(alias = "DOUBLE_BUFFERED")]
pub struct DoubleBuffered {
    chep_rxtxbd_0: ChepRxtxbd0,
    chep_txrxbd_0: ChepTxrxbd0,
}
impl DoubleBuffered {
    #[doc = "0x00 - Channel/endpoint receive buffer descriptor 0"]
    #[inline(always)]
    pub const fn chep_rxtxbd_0(&self) -> &ChepRxtxbd0 {
        &self.chep_rxtxbd_0
    }
    #[doc = "0x04 - Channel/endpoint transmit buffer descriptor 0"]
    #[inline(always)]
    pub const fn chep_txrxbd_0(&self) -> &ChepTxrxbd0 {
        &self.chep_txrxbd_0
    }
}
#[doc = "CHEP_RXTXBD_0 (rw) register accessor: Channel/endpoint receive buffer descriptor 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chep_rxtxbd_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chep_rxtxbd_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep_rxtxbd_0`]
module"]
#[doc(alias = "CHEP_RXTXBD_0")]
pub type ChepRxtxbd0 = crate::Reg<chep_rxtxbd_0::ChepRxtxbd0Spec>;
#[doc = "Channel/endpoint receive buffer descriptor 0"]
pub mod chep_rxtxbd_0;
#[doc = "CHEP_TXRXBD_0 (rw) register accessor: Channel/endpoint transmit buffer descriptor 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chep_txrxbd_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chep_txrxbd_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chep_txrxbd_0`]
module"]
#[doc(alias = "CHEP_TXRXBD_0")]
pub type ChepTxrxbd0 = crate::Reg<chep_txrxbd_0::ChepTxrxbd0Spec>;
#[doc = "Channel/endpoint transmit buffer descriptor 0"]
pub mod chep_txrxbd_0;
