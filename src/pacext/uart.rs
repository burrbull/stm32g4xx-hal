#![allow(unused)]

use crate::{sealed, Sealed};

use super::*;
use crate::pac::lpuart1;
use crate::pac::uart4;
use crate::pac::usart1;
use crate::serial::StopBits;

type NoReturn = ();

pub trait CommonRB: Sealed {
    type CR1rs: reg::Cr1R + reg::Cr1W;
    fn cr1(&self) -> &Reg<Self::CR1rs>;
    type CR2rs: reg::Cr2R + reg::Cr2W;
    fn cr2(&self) -> &Reg<Self::CR2rs>;
    type CR3rs: reg::Cr3R + reg::Cr3W;
    fn cr3(&self) -> &Reg<Self::CR3rs>;
    type BRRrs: RegisterSpec<Ux = u32> + Readable + Writable + Resettable + Sized;
    fn brr(&self) -> &Reg<Self::BRRrs>;
    type RQRrs: reg::Rqr;
    fn rqr(&self) -> &Reg<Self::RQRrs>;
    type ISRrs: reg::Isr;
    fn isr(&self) -> &Reg<Self::ISRrs>;
    type ICRrs: reg::Icr;
    fn icr(&self) -> &Reg<Self::ICRrs>;
    fn rdr(&self) -> &usart1::RDR;
    fn tdr(&self) -> &usart1::TDR;
    fn presc(&self) -> &usart1::PRESC;
}
pub trait UartRB:
    CommonRB<
    RQRrs = usart1::rqr::RQRrs,
    BRRrs = usart1::brr::BRRrs,
    CR1rs: reg::UCr1R + reg::UCr1W,
    CR2rs: reg::UCr2R + reg::UCr2W,
    CR3rs: reg::UCr3R + reg::UCr3W,
    ISRrs: reg::UIsr,
    ICRrs: reg::UIcr,
>
{
    type GTPRrs: reg::GtprR + reg::GtprW;
    fn gtpr(&self) -> &Reg<Self::GTPRrs>;
    fn rtor(&self) -> &usart1::RTOR;
}

wrap_r! {
    pub trait Cr1R {
        fn ue(&self) -> usart1::cr1::UE_R;
        fn uesm(&self) -> usart1::cr1::UESM_R;
        fn re(&self) -> usart1::cr1::RE_R;
        fn te(&self) -> usart1::cr1::TE_R;
        fn idleie(&self) -> usart1::cr1::IDLEIE_R;
        fn rxneie(&self) -> usart1::cr1::RXNEIE_R;
        fn tcie(&self) -> usart1::cr1::TCIE_R;
        fn txeie(&self) -> usart1::cr1::TXEIE_R;
        fn peie(&self) -> usart1::cr1::PEIE_R;
        fn ps(&self) -> usart1::cr1::PS_R;
        fn pce(&self) -> usart1::cr1::PCE_R;
        fn wake(&self) -> usart1::cr1::WAKE_R;
        fn m0(&self) -> usart1::cr1::M0_R;
        fn mme(&self) -> usart1::cr1::MME_R;
        fn cmie(&self) -> usart1::cr1::CMIE_R;
        fn dedt(&self) -> usart1::cr1::DEDT_R;
        fn deat(&self) -> usart1::cr1::DEAT_R;
        fn m1(&self) -> usart1::cr1::M1_R;
        fn fifoen(&self) -> usart1::cr1::FIFOEN_R;
        fn txfeie(&self) -> usart1::cr1::TXFEIE_R;
        fn rxffie(&self) -> usart1::cr1::RXFFIE_R;
    }
}
wrap_r! {
    pub trait UCr1R {
        fn over8(&self) -> usart1::cr1::OVER8_R;
        fn rtoie(&self) -> usart1::cr1::RTOIE_R;
    }
}
wrap_w! {
    pub trait Cr1W {
        fn ue(&mut self) -> usart1::cr1::UE_W<'_, REG>;
        fn uesm(&mut self) -> usart1::cr1::UESM_W<'_, REG>;
        fn re(&mut self) -> usart1::cr1::RE_W<'_, REG>;
        fn te(&mut self) -> usart1::cr1::TE_W<'_, REG>;
        fn idleie(&mut self) -> usart1::cr1::IDLEIE_W<'_, REG>;
        fn rxneie(&mut self) -> usart1::cr1::RXNEIE_W<'_, REG>;
        fn tcie(&mut self) -> usart1::cr1::TCIE_W<'_, REG>;
        fn txeie(&mut self) -> usart1::cr1::TXEIE_W<'_, REG>;
        fn peie(&mut self) -> usart1::cr1::PEIE_W<'_, REG>;
        fn ps(&mut self) -> usart1::cr1::PS_W<'_, REG>;
        fn pce(&mut self) -> usart1::cr1::PCE_W<'_, REG>;
        fn wake(&mut self) -> usart1::cr1::WAKE_W<'_, REG>;
        fn m0(&mut self) -> usart1::cr1::M0_W<'_, REG>;
        fn mme(&mut self) -> usart1::cr1::MME_W<'_, REG>;
        fn cmie(&mut self) -> usart1::cr1::CMIE_W<'_, REG>;
        fn dedt(&mut self) -> usart1::cr1::DEDT_W<'_, REG>;
        fn deat(&mut self) -> usart1::cr1::DEAT_W<'_, REG>;
        fn m1(&mut self) -> usart1::cr1::M1_W<'_, REG>;
        fn fifoen(&mut self) -> usart1::cr1::FIFOEN_W<'_, REG>;
        fn txfeie(&mut self) -> usart1::cr1::TXFEIE_W<'_, REG>;
        fn rxffie(&mut self) -> usart1::cr1::RXFFIE_W<'_, REG>;
    }
}
wrap_w! {
    pub trait UCr1W {
        fn over8(&mut self) -> usart1::cr1::OVER8_W<'_, REG>;
        fn rtoie(&mut self) -> usart1::cr1::RTOIE_W<'_, REG>;
    }
}

wrap_r! {
    pub trait Cr2R {
        fn addm7(&self) -> usart1::cr2::ADDM7_R;
        fn swap(&self) -> usart1::cr2::SWAP_R;
        fn rxinv(&self) -> usart1::cr2::RXINV_R;
        fn txinv(&self) -> usart1::cr2::TXINV_R;
        fn datainv(&self) -> usart1::cr2::DATAINV_R;
        fn msbfirst(&self) -> usart1::cr2::MSBFIRST_R;
        fn add(&self) -> usart1::cr2::ADD_R;
    }
}
wrap_r! {
    pub trait UCr2R {
        fn lbdl(&self) -> usart1::cr2::LBDL_R;
        fn lbdie(&self) -> usart1::cr2::LBDIE_R;
        fn stop(&self) -> usart1::cr2::STOP_R;
        fn linen(&self) -> usart1::cr2::LINEN_R;
        fn abren(&self) -> usart1::cr2::ABREN_R;
        fn abrmod(&self) -> usart1::cr2::ABRMOD_R;
        fn rtoen(&self) -> usart1::cr2::RTOEN_R;
    }
}
wrap_w! {
    pub trait Cr2W {
        fn addm7(&mut self) -> usart1::cr2::ADDM7_W<'_, REG>;
        fn swap(&mut self) -> usart1::cr2::SWAP_W<'_, REG>;
        fn rxinv(&mut self) -> usart1::cr2::RXINV_W<'_, REG>;
        fn txinv(&mut self) -> usart1::cr2::TXINV_W<'_, REG>;
        fn datainv(&mut self) -> usart1::cr2::DATAINV_W<'_, REG>;
        fn msbfirst(&mut self) -> usart1::cr2::MSBFIRST_W<'_, REG>;
        fn add(&mut self) -> usart1::cr2::ADD_W<'_, REG>;
        fn set_stop(&mut self, bits: StopBits) -> NoReturn;
    }
}
wrap_w! {
    pub trait UCr2W {
        fn lbdl(&mut self) -> usart1::cr2::LBDL_W<'_, REG>;
        fn lbdie(&mut self) -> usart1::cr2::LBDIE_W<'_, REG>;
        fn stop(&mut self) -> usart1::cr2::STOP_W<'_, REG>;
        fn linen(&mut self) -> usart1::cr2::LINEN_W<'_, REG>;
        fn abren(&mut self) -> usart1::cr2::ABREN_W<'_, REG>;
        fn abrmod(&mut self) -> usart1::cr2::ABRMOD_W<'_, REG>;
        fn rtoen(&mut self) -> usart1::cr2::RTOEN_W<'_, REG>;
    }
}

wrap_r! {
    pub trait Cr3R {
        fn eie(&self) -> usart1::cr3::EIE_R;
        fn hdsel(&self) -> usart1::cr3::HDSEL_R;
        fn dmar(&self) -> usart1::cr3::DMAR_R;
        fn dmat(&self) -> usart1::cr3::DMAT_R;
        fn rtse(&self) -> usart1::cr3::RTSE_R;
        fn ctse(&self) -> usart1::cr3::CTSE_R;
        fn ctsie(&self) -> usart1::cr3::CTSIE_R;
        fn ovrdis(&self) -> usart1::cr3::OVRDIS_R;
        fn ddre(&self) -> usart1::cr3::DDRE_R;
        fn dem(&self) -> usart1::cr3::DEM_R;
        fn dep(&self) -> usart1::cr3::DEP_R;
        fn wus(&self) -> usart1::cr3::WUS_R;
        fn wufie(&self) -> usart1::cr3::WUFIE_R;
        fn txftie(&self) -> usart1::cr3::TXFTIE_R;
        fn rxftcfg(&self) -> usart1::cr3::RXFTCFG_R;
        fn rxftie(&self) -> usart1::cr3::RXFTIE_R;
        fn txftcfg(&self) -> usart1::cr3::TXFTCFG_R;
    }
}
wrap_r! {
    pub trait UCr3R {
        fn iren(&self) -> usart1::cr3::IREN_R;
        fn irlp(&self) -> usart1::cr3::IRLP_R;
        fn onebit(&self) -> usart1::cr3::ONEBIT_R;
    }
}
wrap_w! {
    pub trait Cr3W {
        fn eie(&mut self) -> usart1::cr3::EIE_W<'_, REG>;
        fn hdsel(&mut self) -> usart1::cr3::HDSEL_W<'_, REG>;
        fn dmar(&mut self) -> usart1::cr3::DMAR_W<'_, REG>;
        fn dmat(&mut self) -> usart1::cr3::DMAT_W<'_, REG>;
        fn rtse(&mut self) -> usart1::cr3::RTSE_W<'_, REG>;
        fn ctse(&mut self) -> usart1::cr3::CTSE_W<'_, REG>;
        fn ctsie(&mut self) -> usart1::cr3::CTSIE_W<'_, REG>;
        fn ovrdis(&mut self) -> usart1::cr3::OVRDIS_W<'_, REG>;
        fn ddre(&mut self) -> usart1::cr3::DDRE_W<'_, REG>;
        fn dem(&mut self) -> usart1::cr3::DEM_W<'_, REG>;
        fn dep(&mut self) -> usart1::cr3::DEP_W<'_, REG>;
        fn wus(&mut self) -> usart1::cr3::WUS_W<'_, REG>;
        fn wufie(&mut self) -> usart1::cr3::WUFIE_W<'_, REG>;
        fn txftie(&mut self) -> usart1::cr3::TXFTIE_W<'_, REG>;
        fn rxftcfg(&mut self) -> usart1::cr3::RXFTCFG_W<'_, REG>;
        fn rxftie(&mut self) -> usart1::cr3::RXFTIE_W<'_, REG>;
        fn txftcfg(&mut self) -> usart1::cr3::TXFTCFG_W<'_, REG>;
    }
}
wrap_w! {
    pub trait UCr3W {
        fn iren(&mut self) -> usart1::cr3::IREN_W<'_, REG>;
        fn irlp(&mut self) -> usart1::cr3::IRLP_W<'_, REG>;
        fn onebit(&mut self) -> usart1::cr3::ONEBIT_W<'_, REG>;
    }
}

wrap_w! {
    pub trait Rqr {
        fn sbkrq(&mut self) -> usart1::rqr::SBKRQ_W<'_, REG>;
        fn mmrq(&mut self) -> usart1::rqr::MMRQ_W<'_, REG>;
        fn rxfrq(&mut self) -> usart1::rqr::RXFRQ_W<'_, REG>;
        fn txfrq(&mut self) -> usart1::rqr::TXFRQ_W<'_, REG>;
    }
}

wrap_r! {
    pub trait Isr {
        fn pe(&self) -> usart1::isr::PE_R;
        fn fe(&self) -> usart1::isr::FE_R;
        fn nf(&self) -> usart1::isr::NF_R;
        fn ore(&self) -> usart1::isr::ORE_R;
        fn idle(&self) -> usart1::isr::IDLE_R;
        fn rxne(&self) -> usart1::isr::RXNE_R;
        fn tc(&self) -> usart1::isr::TC_R;
        fn txe(&self) -> usart1::isr::TXE_R;
        fn ctsif(&self) -> usart1::isr::CTSIF_R;
        fn cts(&self) -> usart1::isr::CTS_R;
        fn busy(&self) -> usart1::isr::BUSY_R;
        fn cmf(&self) -> usart1::isr::CMF_R;
        fn sbkf(&self) -> usart1::isr::SBKF_R;
        fn rwu(&self) -> usart1::isr::RWU_R;
        fn wuf(&self) -> usart1::isr::WUF_R;
        fn teack(&self) -> usart1::isr::TEACK_R;
        fn reack(&self) -> usart1::isr::REACK_R;
        fn txfe(&self) -> usart1::isr::TXFE_R;
        fn rxff(&self) -> usart1::isr::RXFF_R;
        fn rxft(&self) -> usart1::isr::RXFT_R;
        fn txft(&self) -> usart1::isr::TXFT_R;
    }
}
wrap_r! {
    pub trait UIsr {
        fn lbdf(&self) -> usart1::isr::LBDF_R;
        fn rtof(&self) -> usart1::isr::RTOF_R;
        fn abre(&self) -> usart1::isr::ABRE_R;
        fn abrf(&self) -> usart1::isr::ABRF_R;
    }
}

wrap_w! {
    pub trait Icr {
        fn pecf(&mut self) -> usart1::icr::PECF_W<'_, REG>;
        fn fecf(&mut self) -> usart1::icr::FECF_W<'_, REG>;
        fn ncf(&mut self) -> usart1::icr::NCF_W<'_, REG>;
        fn orecf(&mut self) -> usart1::icr::ORECF_W<'_, REG>;
        fn idlecf(&mut self) -> usart1::icr::IDLECF_W<'_, REG>;
        fn tccf(&mut self) -> usart1::icr::TCCF_W<'_, REG>;
        fn ctscf(&mut self) -> usart1::icr::CTSCF_W<'_, REG>;
        fn cmcf(&mut self) -> usart1::icr::CMCF_W<'_, REG>;
        fn wucf(&mut self) -> usart1::icr::WUCF_W<'_, REG>;
    }
}
wrap_w! {
    pub trait UIcr {
        fn txfecf(&mut self) -> usart1::icr::TXFECF_W<'_, REG>;
        fn tcbgtcf(&mut self) -> usart1::icr::TCBGTCF_W<'_, REG>;
        fn lbdcf(&mut self) -> usart1::icr::LBDCF_W<'_, REG>;
        fn rtocf(&mut self) -> usart1::icr::RTOCF_W<'_, REG>;
    }
}

wrap_r! {
    pub trait GtprR {
        fn psc(&self) -> usart1::gtpr::PSC_R;
    }
}
wrap_w! {
    pub trait GtprW {
        fn psc(&mut self) -> usart1::gtpr::PSC_W<'_, REG>;
    }
}

mod reg {
    use super::*;
    pub trait Cr1R: RegisterSpec<Ux = u32> + Readable + Sized {
        fn ue(r: &R<Self>) -> usart1::cr1::UE_R;
        fn uesm(r: &R<Self>) -> usart1::cr1::UESM_R;
        fn re(r: &R<Self>) -> usart1::cr1::RE_R;
        fn te(r: &R<Self>) -> usart1::cr1::TE_R;
        fn idleie(r: &R<Self>) -> usart1::cr1::IDLEIE_R;
        fn rxneie(r: &R<Self>) -> usart1::cr1::RXNEIE_R;
        fn tcie(r: &R<Self>) -> usart1::cr1::TCIE_R;
        fn txeie(r: &R<Self>) -> usart1::cr1::TXEIE_R;
        fn peie(r: &R<Self>) -> usart1::cr1::PEIE_R;
        fn ps(r: &R<Self>) -> usart1::cr1::PS_R;
        fn pce(r: &R<Self>) -> usart1::cr1::PCE_R;
        fn wake(r: &R<Self>) -> usart1::cr1::WAKE_R;
        fn m0(r: &R<Self>) -> usart1::cr1::M0_R;
        fn mme(r: &R<Self>) -> usart1::cr1::MME_R;
        fn cmie(r: &R<Self>) -> usart1::cr1::CMIE_R;
        fn dedt(r: &R<Self>) -> usart1::cr1::DEDT_R;
        fn deat(r: &R<Self>) -> usart1::cr1::DEAT_R;
        fn m1(r: &R<Self>) -> usart1::cr1::M1_R;
        fn fifoen(r: &R<Self>) -> usart1::cr1::FIFOEN_R;
        fn txfeie(r: &R<Self>) -> usart1::cr1::TXFEIE_R;
        fn rxffie(r: &R<Self>) -> usart1::cr1::RXFFIE_R;
    }
    pub trait UCr1R: Cr1R {
        fn over8(r: &R<Self>) -> usart1::cr1::OVER8_R;
        fn rtoie(r: &R<Self>) -> usart1::cr1::RTOIE_R;
    }
    pub trait Cr1W: RegisterSpec<Ux = u32> + Writable + Resettable + Sized {
        fn ue(w: &mut W<Self>) -> usart1::cr1::UE_W<'_, Self>;
        fn uesm(w: &mut W<Self>) -> usart1::cr1::UESM_W<'_, Self>;
        fn re(w: &mut W<Self>) -> usart1::cr1::RE_W<'_, Self>;
        fn te(w: &mut W<Self>) -> usart1::cr1::TE_W<'_, Self>;
        fn idleie(w: &mut W<Self>) -> usart1::cr1::IDLEIE_W<'_, Self>;
        fn rxneie(w: &mut W<Self>) -> usart1::cr1::RXNEIE_W<'_, Self>;
        fn tcie(w: &mut W<Self>) -> usart1::cr1::TCIE_W<'_, Self>;
        fn txeie(w: &mut W<Self>) -> usart1::cr1::TXEIE_W<'_, Self>;
        fn peie(w: &mut W<Self>) -> usart1::cr1::PEIE_W<'_, Self>;
        fn ps(w: &mut W<Self>) -> usart1::cr1::PS_W<'_, Self>;
        fn pce(w: &mut W<Self>) -> usart1::cr1::PCE_W<'_, Self>;
        fn wake(w: &mut W<Self>) -> usart1::cr1::WAKE_W<'_, Self>;
        fn m0(w: &mut W<Self>) -> usart1::cr1::M0_W<'_, Self>;
        fn mme(w: &mut W<Self>) -> usart1::cr1::MME_W<'_, Self>;
        fn cmie(w: &mut W<Self>) -> usart1::cr1::CMIE_W<'_, Self>;
        fn dedt(w: &mut W<Self>) -> usart1::cr1::DEDT_W<'_, Self>;
        fn deat(w: &mut W<Self>) -> usart1::cr1::DEAT_W<'_, Self>;
        fn m1(w: &mut W<Self>) -> usart1::cr1::M1_W<'_, Self>;
        fn fifoen(w: &mut W<Self>) -> usart1::cr1::FIFOEN_W<'_, Self>;
        fn txfeie(w: &mut W<Self>) -> usart1::cr1::TXFEIE_W<'_, Self>;
        fn rxffie(w: &mut W<Self>) -> usart1::cr1::RXFFIE_W<'_, Self>;
    }
    pub trait UCr1W: Cr1W {
        fn over8(w: &mut W<Self>) -> usart1::cr1::OVER8_W<'_, Self>;
        fn rtoie(w: &mut W<Self>) -> usart1::cr1::RTOIE_W<'_, Self>;
    }

    pub trait Cr2R: RegisterSpec<Ux = u32> + Readable + Sized {
        fn addm7(r: &R<Self>) -> usart1::cr2::ADDM7_R;
        fn swap(r: &R<Self>) -> usart1::cr2::SWAP_R;
        fn rxinv(r: &R<Self>) -> usart1::cr2::RXINV_R;
        fn txinv(r: &R<Self>) -> usart1::cr2::TXINV_R;
        fn datainv(r: &R<Self>) -> usart1::cr2::DATAINV_R;
        fn msbfirst(r: &R<Self>) -> usart1::cr2::MSBFIRST_R;
        fn add(r: &R<Self>) -> usart1::cr2::ADD_R;
    }
    pub trait UCr2R: Cr2R {
        fn lbdl(r: &R<Self>) -> usart1::cr2::LBDL_R;
        fn lbdie(r: &R<Self>) -> usart1::cr2::LBDIE_R;
        fn stop(r: &R<Self>) -> usart1::cr2::STOP_R;
        fn linen(r: &R<Self>) -> usart1::cr2::LINEN_R;
        fn abren(r: &R<Self>) -> usart1::cr2::ABREN_R;
        fn abrmod(r: &R<Self>) -> usart1::cr2::ABRMOD_R;
        fn rtoen(r: &R<Self>) -> usart1::cr2::RTOEN_R;
    }
    pub trait Cr2W: RegisterSpec<Ux = u32> + Writable + Resettable + Sized {
        fn addm7(w: &mut W<Self>) -> usart1::cr2::ADDM7_W<'_, Self>;
        fn swap(w: &mut W<Self>) -> usart1::cr2::SWAP_W<'_, Self>;
        fn rxinv(w: &mut W<Self>) -> usart1::cr2::RXINV_W<'_, Self>;
        fn txinv(w: &mut W<Self>) -> usart1::cr2::TXINV_W<'_, Self>;
        fn datainv(w: &mut W<Self>) -> usart1::cr2::DATAINV_W<'_, Self>;
        fn msbfirst(w: &mut W<Self>) -> usart1::cr2::MSBFIRST_W<'_, Self>;
        fn add(w: &mut W<Self>) -> usart1::cr2::ADD_W<'_, Self>;
        fn set_stop(w: &mut W<Self>, bits: StopBits);
    }
    pub trait UCr2W: Cr2W {
        fn lbdl(w: &mut W<Self>) -> usart1::cr2::LBDL_W<'_, Self>;
        fn lbdie(w: &mut W<Self>) -> usart1::cr2::LBDIE_W<'_, Self>;
        fn stop(w: &mut W<Self>) -> usart1::cr2::STOP_W<'_, Self>;
        fn linen(w: &mut W<Self>) -> usart1::cr2::LINEN_W<'_, Self>;
        fn abren(w: &mut W<Self>) -> usart1::cr2::ABREN_W<'_, Self>;
        fn abrmod(w: &mut W<Self>) -> usart1::cr2::ABRMOD_W<'_, Self>;
        fn rtoen(w: &mut W<Self>) -> usart1::cr2::RTOEN_W<'_, Self>;
    }

    pub trait Cr3R: RegisterSpec<Ux = u32> + Readable + Sized {
        fn eie(r: &R<Self>) -> usart1::cr3::EIE_R;
        fn hdsel(r: &R<Self>) -> usart1::cr3::HDSEL_R;
        fn dmar(r: &R<Self>) -> usart1::cr3::DMAR_R;
        fn dmat(r: &R<Self>) -> usart1::cr3::DMAT_R;
        fn rtse(r: &R<Self>) -> usart1::cr3::RTSE_R;
        fn ctse(r: &R<Self>) -> usart1::cr3::CTSE_R;
        fn ctsie(r: &R<Self>) -> usart1::cr3::CTSIE_R;
        fn ovrdis(r: &R<Self>) -> usart1::cr3::OVRDIS_R;
        fn ddre(r: &R<Self>) -> usart1::cr3::DDRE_R;
        fn dem(r: &R<Self>) -> usart1::cr3::DEM_R;
        fn dep(r: &R<Self>) -> usart1::cr3::DEP_R;
        fn wus(r: &R<Self>) -> usart1::cr3::WUS_R;
        fn wufie(r: &R<Self>) -> usart1::cr3::WUFIE_R;
        fn txftie(r: &R<Self>) -> usart1::cr3::TXFTIE_R;
        fn rxftcfg(r: &R<Self>) -> usart1::cr3::RXFTCFG_R;
        fn rxftie(r: &R<Self>) -> usart1::cr3::RXFTIE_R;
        fn txftcfg(r: &R<Self>) -> usart1::cr3::TXFTCFG_R;
    }
    pub trait UCr3R: Cr3R {
        fn iren(r: &R<Self>) -> usart1::cr3::IREN_R;
        fn irlp(r: &R<Self>) -> usart1::cr3::IRLP_R;
        fn onebit(r: &R<Self>) -> usart1::cr3::ONEBIT_R;
    }
    pub trait Cr3W: RegisterSpec<Ux = u32> + Writable + Resettable + Sized {
        fn eie(w: &mut W<Self>) -> usart1::cr3::EIE_W<'_, Self>;
        fn hdsel(w: &mut W<Self>) -> usart1::cr3::HDSEL_W<'_, Self>;
        fn dmar(w: &mut W<Self>) -> usart1::cr3::DMAR_W<'_, Self>;
        fn dmat(w: &mut W<Self>) -> usart1::cr3::DMAT_W<'_, Self>;
        fn rtse(w: &mut W<Self>) -> usart1::cr3::RTSE_W<'_, Self>;
        fn ctse(w: &mut W<Self>) -> usart1::cr3::CTSE_W<'_, Self>;
        fn ctsie(w: &mut W<Self>) -> usart1::cr3::CTSIE_W<'_, Self>;
        fn ovrdis(w: &mut W<Self>) -> usart1::cr3::OVRDIS_W<'_, Self>;
        fn ddre(w: &mut W<Self>) -> usart1::cr3::DDRE_W<'_, Self>;
        fn dem(w: &mut W<Self>) -> usart1::cr3::DEM_W<'_, Self>;
        fn dep(w: &mut W<Self>) -> usart1::cr3::DEP_W<'_, Self>;
        fn wus(w: &mut W<Self>) -> usart1::cr3::WUS_W<'_, Self>;
        fn wufie(w: &mut W<Self>) -> usart1::cr3::WUFIE_W<'_, Self>;
        fn txftie(w: &mut W<Self>) -> usart1::cr3::TXFTIE_W<'_, Self>;
        fn rxftcfg(w: &mut W<Self>) -> usart1::cr3::RXFTCFG_W<'_, Self>;
        fn rxftie(w: &mut W<Self>) -> usart1::cr3::RXFTIE_W<'_, Self>;
        fn txftcfg(w: &mut W<Self>) -> usart1::cr3::TXFTCFG_W<'_, Self>;
    }
    pub trait UCr3W: Cr3W {
        fn iren(w: &mut W<Self>) -> usart1::cr3::IREN_W<'_, Self>;
        fn irlp(w: &mut W<Self>) -> usart1::cr3::IRLP_W<'_, Self>;
        fn onebit(w: &mut W<Self>) -> usart1::cr3::ONEBIT_W<'_, Self>;
    }

    pub trait Rqr: RegisterSpec<Ux = u32> + Writable + Resettable + Sized {
        fn sbkrq(w: &mut W<Self>) -> usart1::rqr::SBKRQ_W<'_, Self>;
        fn mmrq(w: &mut W<Self>) -> usart1::rqr::MMRQ_W<'_, Self>;
        fn rxfrq(w: &mut W<Self>) -> usart1::rqr::RXFRQ_W<'_, Self>;
        fn txfrq(w: &mut W<Self>) -> usart1::rqr::TXFRQ_W<'_, Self>;
    }

    pub trait Isr: RegisterSpec<Ux = u32> + Readable + Sized {
        fn pe(r: &R<Self>) -> usart1::isr::PE_R;
        fn fe(r: &R<Self>) -> usart1::isr::FE_R;
        fn nf(r: &R<Self>) -> usart1::isr::NF_R;
        fn ore(r: &R<Self>) -> usart1::isr::ORE_R;
        fn idle(r: &R<Self>) -> usart1::isr::IDLE_R;
        fn rxne(r: &R<Self>) -> usart1::isr::RXNE_R;
        fn tc(r: &R<Self>) -> usart1::isr::TC_R;
        fn txe(r: &R<Self>) -> usart1::isr::TXE_R;
        fn ctsif(r: &R<Self>) -> usart1::isr::CTSIF_R;
        fn cts(r: &R<Self>) -> usart1::isr::CTS_R;
        fn busy(r: &R<Self>) -> usart1::isr::BUSY_R;
        fn cmf(r: &R<Self>) -> usart1::isr::CMF_R;
        fn sbkf(r: &R<Self>) -> usart1::isr::SBKF_R;
        fn rwu(r: &R<Self>) -> usart1::isr::RWU_R;
        fn wuf(r: &R<Self>) -> usart1::isr::WUF_R;
        fn teack(r: &R<Self>) -> usart1::isr::TEACK_R;
        fn reack(r: &R<Self>) -> usart1::isr::REACK_R;
        fn txfe(r: &R<Self>) -> usart1::isr::TXFE_R;
        fn rxff(r: &R<Self>) -> usart1::isr::RXFF_R;
        fn rxft(r: &R<Self>) -> usart1::isr::RXFT_R;
        fn txft(r: &R<Self>) -> usart1::isr::TXFT_R;
    }
    pub trait UIsr: Isr {
        fn lbdf(r: &R<Self>) -> usart1::isr::LBDF_R;
        fn rtof(r: &R<Self>) -> usart1::isr::RTOF_R;
        fn abre(r: &R<Self>) -> usart1::isr::ABRE_R;
        fn abrf(r: &R<Self>) -> usart1::isr::ABRF_R;
    }
    pub trait Icr: RegisterSpec<Ux = u32> + Writable + Resettable + Sized {
        fn pecf(w: &mut W<Self>) -> usart1::icr::PECF_W<'_, Self>;
        fn fecf(w: &mut W<Self>) -> usart1::icr::FECF_W<'_, Self>;
        fn ncf(w: &mut W<Self>) -> usart1::icr::NCF_W<'_, Self>;
        fn orecf(w: &mut W<Self>) -> usart1::icr::ORECF_W<'_, Self>;
        fn idlecf(w: &mut W<Self>) -> usart1::icr::IDLECF_W<'_, Self>;
        fn tccf(w: &mut W<Self>) -> usart1::icr::TCCF_W<'_, Self>;
        fn ctscf(w: &mut W<Self>) -> usart1::icr::CTSCF_W<'_, Self>;
        fn cmcf(w: &mut W<Self>) -> usart1::icr::CMCF_W<'_, Self>;
        fn wucf(w: &mut W<Self>) -> usart1::icr::WUCF_W<'_, Self>;
    }
    pub trait UIcr: Icr {
        fn txfecf(w: &mut W<Self>) -> usart1::icr::TXFECF_W<'_, Self>;
        fn tcbgtcf(w: &mut W<Self>) -> usart1::icr::TCBGTCF_W<'_, Self>;
        fn lbdcf(w: &mut W<Self>) -> usart1::icr::LBDCF_W<'_, Self>;
        fn rtocf(w: &mut W<Self>) -> usart1::icr::RTOCF_W<'_, Self>;
    }

    pub trait GtprR: RegisterSpec<Ux = u32> + Readable + Sized {
        fn psc(r: &R<Self>) -> usart1::gtpr::PSC_R;
    }
    pub trait GtprW: RegisterSpec<Ux = u32> + Writable + Resettable + Sized {
        fn psc(w: &mut W<Self>) -> usart1::gtpr::PSC_W<'_, Self>;
    }
}

macro_rules! impl_ext {
    ($uart:ident) => {
        impl Sealed for $uart::RegisterBlock {}
        impl CommonRB for $uart::RegisterBlock {
            type CR1rs = $uart::cr1::CR1rs;
            type CR2rs = $uart::cr2::CR2rs;
            type CR3rs = $uart::cr3::CR3rs;
            type BRRrs = $uart::brr::BRRrs;
            type RQRrs = $uart::rqr::RQRrs;
            type ISRrs = $uart::isr::ISRrs;
            type ICRrs = $uart::icr::ICRrs;
            impl_reg! {
                cr1 -> &Reg<Self::CR1rs>;
                cr2 -> &Reg<Self::CR2rs>;
                cr3 -> &Reg<Self::CR3rs>;
                brr -> &Reg<Self::BRRrs>;
                rqr -> &Reg<Self::RQRrs>;
                isr -> &Reg<Self::ISRrs>;
                icr -> &Reg<Self::ICRrs>;
                rdr -> &usart1::RDR;
                tdr -> &usart1::TDR;
                presc -> &usart1::PRESC;
            }
        }
        impl reg::Cr1R for $uart::cr1::CR1rs {
            impl_read! {
                ue -> usart1::cr1::UE_R;
                uesm -> usart1::cr1::UESM_R;
                re -> usart1::cr1::RE_R;
                te -> usart1::cr1::TE_R;
                idleie -> usart1::cr1::IDLEIE_R;
                rxneie -> usart1::cr1::RXNEIE_R;
                tcie -> usart1::cr1::TCIE_R;
                txeie -> usart1::cr1::TXEIE_R;
                peie -> usart1::cr1::PEIE_R;
                ps -> usart1::cr1::PS_R;
                pce -> usart1::cr1::PCE_R;
                wake -> usart1::cr1::WAKE_R;
                m0 -> usart1::cr1::M0_R;
                mme -> usart1::cr1::MME_R;
                cmie -> usart1::cr1::CMIE_R;
                dedt -> usart1::cr1::DEDT_R;
                deat -> usart1::cr1::DEAT_R;
                m1 -> usart1::cr1::M1_R;
                fifoen -> usart1::cr1::FIFOEN_R;
                txfeie -> usart1::cr1::TXFEIE_R;
                rxffie -> usart1::cr1::RXFFIE_R;
            }
        }
        impl reg::Cr1W for $uart::cr1::CR1rs {
            impl_write! {
                ue -> usart1::cr1::UE_W<'_, Self>;
                uesm -> usart1::cr1::UESM_W<'_, Self>;
                re -> usart1::cr1::RE_W<'_, Self>;
                te -> usart1::cr1::TE_W<'_, Self>;
                idleie -> usart1::cr1::IDLEIE_W<'_, Self>;
                rxneie -> usart1::cr1::RXNEIE_W<'_, Self>;
                tcie -> usart1::cr1::TCIE_W<'_, Self>;
                txeie -> usart1::cr1::TXEIE_W<'_, Self>;
                peie -> usart1::cr1::PEIE_W<'_, Self>;
                ps -> usart1::cr1::PS_W<'_, Self>;
                pce -> usart1::cr1::PCE_W<'_, Self>;
                wake -> usart1::cr1::WAKE_W<'_, Self>;
                m0 -> usart1::cr1::M0_W<'_, Self>;
                mme -> usart1::cr1::MME_W<'_, Self>;
                cmie -> usart1::cr1::CMIE_W<'_, Self>;
                dedt -> usart1::cr1::DEDT_W<'_, Self>;
                deat -> usart1::cr1::DEAT_W<'_, Self>;
                m1 -> usart1::cr1::M1_W<'_, Self>;
                fifoen -> usart1::cr1::FIFOEN_W<'_, Self>;
                txfeie -> usart1::cr1::TXFEIE_W<'_, Self>;
                rxffie -> usart1::cr1::RXFFIE_W<'_, Self>;
            }
        }

        impl reg::Cr2R for $uart::cr2::CR2rs {
            impl_read! {
                addm7 -> usart1::cr2::ADDM7_R;
                swap -> usart1::cr2::SWAP_R;
                rxinv -> usart1::cr2::RXINV_R;
                txinv -> usart1::cr2::TXINV_R;
                datainv -> usart1::cr2::DATAINV_R;
                msbfirst -> usart1::cr2::MSBFIRST_R;
                add -> usart1::cr2::ADD_R;
            }
        }
        impl reg::Cr2W for $uart::cr2::CR2rs {
            impl_write! {
                addm7 -> usart1::cr2::ADDM7_W<'_, Self>;
                swap -> usart1::cr2::SWAP_W<'_, Self>;
                rxinv -> usart1::cr2::RXINV_W<'_, Self>;
                txinv -> usart1::cr2::TXINV_W<'_, Self>;
                datainv -> usart1::cr2::DATAINV_W<'_, Self>;
                msbfirst -> usart1::cr2::MSBFIRST_W<'_, Self>;
                add -> usart1::cr2::ADD_W<'_, Self>;
            }
            #[inline(always)]
            fn set_stop(w: &mut W<Self>, bits: StopBits) {
                w.stop().variant(bits.into());
            }
        }

        impl reg::Cr3R for $uart::cr3::CR3rs {
            impl_read! {
                eie -> usart1::cr3::EIE_R;
                hdsel -> usart1::cr3::HDSEL_R;
                dmar -> usart1::cr3::DMAR_R;
                dmat -> usart1::cr3::DMAT_R;
                rtse -> usart1::cr3::RTSE_R;
                ctse -> usart1::cr3::CTSE_R;
                ctsie -> usart1::cr3::CTSIE_R;
                ovrdis -> usart1::cr3::OVRDIS_R;
                ddre -> usart1::cr3::DDRE_R;
                dem -> usart1::cr3::DEM_R;
                dep -> usart1::cr3::DEP_R;
                wus -> usart1::cr3::WUS_R;
                wufie -> usart1::cr3::WUFIE_R;
                txftie -> usart1::cr3::TXFTIE_R;
                rxftcfg -> usart1::cr3::RXFTCFG_R;
                rxftie -> usart1::cr3::RXFTIE_R;
                txftcfg -> usart1::cr3::TXFTCFG_R;
            }
        }
        impl reg::Cr3W for $uart::cr3::CR3rs {
            impl_write! {
                eie -> usart1::cr3::EIE_W<'_, Self>;
                hdsel -> usart1::cr3::HDSEL_W<'_, Self>;
                dmar -> usart1::cr3::DMAR_W<'_, Self>;
                dmat -> usart1::cr3::DMAT_W<'_, Self>;
                rtse -> usart1::cr3::RTSE_W<'_, Self>;
                ctse -> usart1::cr3::CTSE_W<'_, Self>;
                ctsie -> usart1::cr3::CTSIE_W<'_, Self>;
                ovrdis -> usart1::cr3::OVRDIS_W<'_, Self>;
                ddre -> usart1::cr3::DDRE_W<'_, Self>;
                dem -> usart1::cr3::DEM_W<'_, Self>;
                dep -> usart1::cr3::DEP_W<'_, Self>;
                wus -> usart1::cr3::WUS_W<'_, Self>;
                wufie -> usart1::cr3::WUFIE_W<'_, Self>;
                txftie -> usart1::cr3::TXFTIE_W<'_, Self>;
                rxftcfg -> usart1::cr3::RXFTCFG_W<'_, Self>;
                rxftie -> usart1::cr3::RXFTIE_W<'_, Self>;
                txftcfg -> usart1::cr3::TXFTCFG_W<'_, Self>;
            }
        }

        impl reg::Isr for $uart::isr::ISRrs {
            impl_read! {
                pe -> usart1::isr::PE_R;
                fe -> usart1::isr::FE_R;
                nf -> usart1::isr::NF_R;
                ore -> usart1::isr::ORE_R;
                idle -> usart1::isr::IDLE_R;
                rxne -> usart1::isr::RXNE_R;
                tc -> usart1::isr::TC_R;
                txe -> usart1::isr::TXE_R;
                ctsif -> usart1::isr::CTSIF_R;
                cts -> usart1::isr::CTS_R;
                busy -> usart1::isr::BUSY_R;
                cmf -> usart1::isr::CMF_R;
                sbkf -> usart1::isr::SBKF_R;
                rwu -> usart1::isr::RWU_R;
                wuf -> usart1::isr::WUF_R;
                teack -> usart1::isr::TEACK_R;
                reack -> usart1::isr::REACK_R;
                txfe -> usart1::isr::TXFE_R;
                rxff -> usart1::isr::RXFF_R;
                rxft -> usart1::isr::RXFT_R;
                txft -> usart1::isr::TXFT_R;
            }
        }
        impl reg::Icr for $uart::icr::ICRrs {
            impl_write! {
                pecf -> usart1::icr::PECF_W<'_, Self>;
                fecf -> usart1::icr::FECF_W<'_, Self>;
                ncf -> usart1::icr::NCF_W<'_, Self>;
                orecf -> usart1::icr::ORECF_W<'_, Self>;
                idlecf -> usart1::icr::IDLECF_W<'_, Self>;
                tccf -> usart1::icr::TCCF_W<'_, Self>;
                ctscf -> usart1::icr::CTSCF_W<'_, Self>;
                cmcf -> usart1::icr::CMCF_W<'_, Self>;
                wucf -> usart1::icr::WUCF_W<'_, Self>;
            }
        }
    };
}
macro_rules! impl_rqr {
    ($uart:ident) => {
        impl reg::Rqr for $uart::rqr::RQRrs {
            impl_write! {
                sbkrq -> usart1::rqr::SBKRQ_W<'_, Self>;
                mmrq -> usart1::rqr::MMRQ_W<'_, Self>;
                rxfrq -> usart1::rqr::RXFRQ_W<'_, Self>;
                txfrq -> usart1::rqr::TXFRQ_W<'_, Self>;
            }
        }
    };
}

macro_rules! impl_uart {
    ($uart:ident) => {
        impl UartRB for $uart::RegisterBlock {
            type GTPRrs = $uart::gtpr::GTPRrs;
            impl_reg! {
                gtpr -> &Reg<Self::GTPRrs>;
                rtor -> &usart1::RTOR;
            }
        }
        impl reg::UCr1R for $uart::cr1::CR1rs {
            impl_read! {
                over8 -> usart1::cr1::OVER8_R;
                rtoie -> usart1::cr1::RTOIE_R;
            }
        }
        impl reg::UCr1W for $uart::cr1::CR1rs {
            impl_write! {
                over8 -> usart1::cr1::OVER8_W<'_, Self>;
                rtoie -> usart1::cr1::RTOIE_W<'_, Self>;
            }
        }
        impl reg::UCr2R for $uart::cr2::CR2rs {
            impl_read! {
                lbdl -> usart1::cr2::LBDL_R;
                lbdie -> usart1::cr2::LBDIE_R;
                stop -> usart1::cr2::STOP_R;
                linen -> usart1::cr2::LINEN_R;
                abren -> usart1::cr2::ABREN_R;
                abrmod -> usart1::cr2::ABRMOD_R;
                rtoen -> usart1::cr2::RTOEN_R;
            }
        }
        impl reg::UCr2W for $uart::cr2::CR2rs {
            impl_write! {
                lbdl -> usart1::cr2::LBDL_W<'_, Self>;
                lbdie -> usart1::cr2::LBDIE_W<'_, Self>;
                stop -> usart1::cr2::STOP_W<'_, Self>;
                linen -> usart1::cr2::LINEN_W<'_, Self>;
                abren -> usart1::cr2::ABREN_W<'_, Self>;
                abrmod -> usart1::cr2::ABRMOD_W<'_, Self>;
                rtoen -> usart1::cr2::RTOEN_W<'_, Self>;
            }
        }
        impl reg::UCr3R for $uart::cr3::CR3rs {
            impl_read! {
                iren -> usart1::cr3::IREN_R;
                irlp -> usart1::cr3::IRLP_R;
                onebit -> usart1::cr3::ONEBIT_R;
            }
        }
        impl reg::UCr3W for $uart::cr3::CR3rs {
            impl_write! {
                iren -> usart1::cr3::IREN_W<'_, Self>;
                irlp -> usart1::cr3::IRLP_W<'_, Self>;
                onebit -> usart1::cr3::ONEBIT_W<'_, Self>;
            }
        }

        impl reg::GtprR for $uart::gtpr::GTPRrs {
            impl_read! {
                psc -> usart1::gtpr::PSC_R;
            }
        }
        impl reg::GtprW for $uart::gtpr::GTPRrs {
            impl_write! {
                psc -> usart1::gtpr::PSC_W<'_, Self>;
            }
        }

        impl reg::UIsr for $uart::isr::ISRrs {
            impl_read! {
                lbdf -> usart1::isr::LBDF_R;
                rtof -> usart1::isr::RTOF_R;
                abre -> usart1::isr::ABRE_R;
                abrf -> usart1::isr::ABRF_R;
            }
        }
        impl reg::UIcr for $uart::icr::ICRrs {
            impl_write! {
                txfecf -> usart1::icr::TXFECF_W<'_, Self>;
                tcbgtcf -> usart1::icr::TCBGTCF_W<'_, Self>;
                lbdcf -> usart1::icr::LBDCF_W<'_, Self>;
                rtocf -> usart1::icr::RTOCF_W<'_, Self>;
            }
        }
    };
}

impl_ext!(usart1);
impl_ext!(uart4);
impl_ext!(lpuart1);

impl_rqr!(usart1);
impl_rqr!(lpuart1);

impl_uart!(usart1);
impl_uart!(uart4);
