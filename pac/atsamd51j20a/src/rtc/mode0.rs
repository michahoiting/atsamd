#[doc = "MODE0 Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u16, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "MODE0 Control A"]
pub mod ctrla;
#[doc = "MODE0 Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlb](ctrlb) module"]
pub type CTRLB = crate::Reg<u16, _CTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB;
#[doc = "`read()` method returns [ctrlb::R](ctrlb::R) reader structure"]
impl crate::Readable for CTRLB {}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](ctrlb::W) writer structure"]
impl crate::Writable for CTRLB {}
#[doc = "MODE0 Control B"]
pub mod ctrlb;
#[doc = "MODE0 Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [evctrl](evctrl) module"]
pub type EVCTRL = crate::Reg<u32, _EVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCTRL;
#[doc = "`read()` method returns [evctrl::R](evctrl::R) reader structure"]
impl crate::Readable for EVCTRL {}
#[doc = "`write(|w| ..)` method takes [evctrl::W](evctrl::W) writer structure"]
impl crate::Writable for EVCTRL {}
#[doc = "MODE0 Event Control"]
pub mod evctrl;
#[doc = "MODE0 Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u16, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "MODE0 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "MODE0 Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u16, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "MODE0 Interrupt Enable Set"]
pub mod intenset;
#[doc = "MODE0 Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u16, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "MODE0 Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Debug Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbgctrl](dbgctrl) module"]
pub type DBGCTRL = crate::Reg<u8, _DBGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGCTRL;
#[doc = "`read()` method returns [dbgctrl::R](dbgctrl::R) reader structure"]
impl crate::Readable for DBGCTRL {}
#[doc = "`write(|w| ..)` method takes [dbgctrl::W](dbgctrl::W) writer structure"]
impl crate::Writable for DBGCTRL {}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "MODE0 Synchronization Busy Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "MODE0 Synchronization Busy Status"]
pub mod syncbusy;
#[doc = "Frequency Correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [freqcorr](freqcorr) module"]
pub type FREQCORR = crate::Reg<u8, _FREQCORR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQCORR;
#[doc = "`read()` method returns [freqcorr::R](freqcorr::R) reader structure"]
impl crate::Readable for FREQCORR {}
#[doc = "`write(|w| ..)` method takes [freqcorr::W](freqcorr::W) writer structure"]
impl crate::Writable for FREQCORR {}
#[doc = "Frequency Correction"]
pub mod freqcorr;
#[doc = "MODE0 Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [count](count) module"]
pub type COUNT = crate::Reg<u32, _COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT;
#[doc = "`read()` method returns [count::R](count::R) reader structure"]
impl crate::Readable for COUNT {}
#[doc = "`write(|w| ..)` method takes [count::W](count::W) writer structure"]
impl crate::Writable for COUNT {}
#[doc = "MODE0 Counter Value"]
pub mod count;
#[doc = "MODE0 Compare n Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp](comp) module"]
pub type COMP = crate::Reg<u32, _COMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP;
#[doc = "`read()` method returns [comp::R](comp::R) reader structure"]
impl crate::Readable for COMP {}
#[doc = "`write(|w| ..)` method takes [comp::W](comp::W) writer structure"]
impl crate::Writable for COMP {}
#[doc = "MODE0 Compare n Value"]
pub mod comp;
#[doc = "General Purpose\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gp](gp) module"]
pub type GP = crate::Reg<u32, _GP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP;
#[doc = "`read()` method returns [gp::R](gp::R) reader structure"]
impl crate::Readable for GP {}
#[doc = "`write(|w| ..)` method takes [gp::W](gp::W) writer structure"]
impl crate::Writable for GP {}
#[doc = "General Purpose"]
pub mod gp;
#[doc = "Tamper Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tampctrl](tampctrl) module"]
pub type TAMPCTRL = crate::Reg<u32, _TAMPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMPCTRL;
#[doc = "`read()` method returns [tampctrl::R](tampctrl::R) reader structure"]
impl crate::Readable for TAMPCTRL {}
#[doc = "`write(|w| ..)` method takes [tampctrl::W](tampctrl::W) writer structure"]
impl crate::Writable for TAMPCTRL {}
#[doc = "Tamper Control"]
pub mod tampctrl;
#[doc = "MODE0 Timestamp\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timestamp](timestamp) module"]
pub type TIMESTAMP = crate::Reg<u32, _TIMESTAMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMESTAMP;
#[doc = "`read()` method returns [timestamp::R](timestamp::R) reader structure"]
impl crate::Readable for TIMESTAMP {}
#[doc = "MODE0 Timestamp"]
pub mod timestamp;
#[doc = "Tamper ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tampid](tampid) module"]
pub type TAMPID = crate::Reg<u32, _TAMPID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMPID;
#[doc = "`read()` method returns [tampid::R](tampid::R) reader structure"]
impl crate::Readable for TAMPID {}
#[doc = "`write(|w| ..)` method takes [tampid::W](tampid::W) writer structure"]
impl crate::Writable for TAMPID {}
#[doc = "Tamper ID"]
pub mod tampid;
#[doc = "Backup\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bkup](bkup) module"]
pub type BKUP = crate::Reg<u32, _BKUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKUP;
#[doc = "`read()` method returns [bkup::R](bkup::R) reader structure"]
impl crate::Readable for BKUP {}
#[doc = "`write(|w| ..)` method takes [bkup::W](bkup::W) writer structure"]
impl crate::Writable for BKUP {}
#[doc = "Backup"]
pub mod bkup;
