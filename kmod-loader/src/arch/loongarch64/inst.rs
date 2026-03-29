#![allow(non_camel_case_types)]
#![allow(unused)]
use bitfield_struct::bitfield;

use crate::arch::{SZ_2K, SZ_128K, SZ_512K};

const INSN_BREAK: u32 = 0x002a0000;

pub const ADDR_IMMMASK_LU52ID: u64 = 0xFFF0000000000000;
pub const ADDR_IMMMASK_LU32ID: u64 = 0x000FFFFF00000000;
pub const ADDR_IMMMASK_LU12IW: u64 = 0x00000000FFFFF000;
pub const ADDR_IMMMASK_ORI: u64 = 0x0000000000000FFF;
pub const ADDR_IMMMASK_ADDU16ID: u64 = 0x00000000FFFF0000;

pub const ADDR_IMMSHIFT_LU52ID: u32 = 52;
pub const ADDR_IMMSBIDX_LU52ID: u32 = 11;
pub const ADDR_IMMSHIFT_LU32ID: u32 = 32;
pub const ADDR_IMMSBIDX_LU32ID: u32 = 19;
pub const ADDR_IMMSHIFT_LU12IW: u32 = 12;
pub const ADDR_IMMSBIDX_LU12IW: u32 = 19;
pub const ADDR_IMMSHIFT_ORI: u32 = 0;
pub const ADDR_IMMSBIDX_ORI: u32 = 63;
pub const ADDR_IMMSHIFT_ADDU16ID: u32 = 16;
pub const ADDR_IMMSBIDX_ADDU16ID: u32 = 15;

macro_rules! ADDR_IMM {
    ($addr:expr, $insn:ident) => {
        $crate::paste::paste! {
            $crate::arch::sign_extend64((($addr & [<ADDR_IMMMASK_ $insn>]) >> [<ADDR_IMMSHIFT_ $insn>]), [<ADDR_IMMSBIDX_ $insn>])
        }
    };
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum loongarch_gpr {
    LOONGARCH_GPR_ZERO = 0,
    LOONGARCH_GPR_RA = 1,
    LOONGARCH_GPR_TP = 2,
    LOONGARCH_GPR_SP = 3,
    LOONGARCH_GPR_A0 = 4, /* Reused as V0 for return value */
    LOONGARCH_GPR_A1,     /* Reused as V1 for return value */
    LOONGARCH_GPR_A2,
    LOONGARCH_GPR_A3,
    LOONGARCH_GPR_A4,
    LOONGARCH_GPR_A5,
    LOONGARCH_GPR_A6,
    LOONGARCH_GPR_A7,
    LOONGARCH_GPR_T0 = 12,
    LOONGARCH_GPR_T1,
    LOONGARCH_GPR_T2,
    LOONGARCH_GPR_T3,
    LOONGARCH_GPR_T4,
    LOONGARCH_GPR_T5,
    LOONGARCH_GPR_T6,
    LOONGARCH_GPR_T7,
    LOONGARCH_GPR_T8,
    LOONGARCH_GPR_FP = 22,
    LOONGARCH_GPR_S0 = 23,
    LOONGARCH_GPR_S1,
    LOONGARCH_GPR_S2,
    LOONGARCH_GPR_S3,
    LOONGARCH_GPR_S4,
    LOONGARCH_GPR_S5,
    LOONGARCH_GPR_S6,
    LOONGARCH_GPR_S7,
    LOONGARCH_GPR_S8,
    LOONGARCH_GPR_MAX,
}

#[bitfield(u32)]
pub struct reg0i15_format {
    #[bits(15)]
    pub immediate: u32,
    #[bits(17)]
    pub opcode: u32,
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct reg0i26_format {
    #[bits(10)]
    pub immediate_h: u32,
    #[bits(16)]
    pub immediate_l: u32,
    #[bits(6)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg1i20_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(20)]
    pub immediate: u32,
    #[bits(7)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg1i21_format {
    #[bits(5)]
    pub immediate_h: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(16)]
    pub immediate_l: u32,
    #[bits(6)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(22)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2i5_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(5)]
    pub immediate: u32,
    #[bits(17)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2i6_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(6)]
    pub immediate: u32,
    #[bits(16)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2i12_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(12)]
    pub immediate: u32,
    #[bits(10)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2i14_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(14)]
    pub immediate: u32,
    #[bits(8)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2i16_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(16)]
    pub immediate: u32,
    #[bits(6)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2bstrd_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(6)]
    lsbd: u32,
    #[bits(6)]
    msbd: u32,
    #[bits(10)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg3_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(5)]
    rk: u32,
    #[bits(17)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg3sa2_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(5)]
    rk: u32,
    #[bits(2)]
    pub immediate: u32,
    #[bits(15)]
    pub opcode: u32,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum reg1i20_op {
    lu12iw_op = 0x0a,
    lu32id_op = 0x0b,
    pcaddi_op = 0x0c,
    pcalau12i_op = 0x0d,
    pcaddu12i_op = 0x0e,
    pcaddu18i_op = 0x0f,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum reg2i12_op {
    addiw_op = 0x0a,
    addid_op = 0x0b,
    lu52id_op = 0x0c,
    andi_op = 0x0d,
    ori_op = 0x0e,
    xori_op = 0x0f,
    ldb_op = 0xa0,
    ldh_op = 0xa1,
    ldw_op = 0xa2,
    ldd_op = 0xa3,
    stb_op = 0xa4,
    sth_op = 0xa5,
    stw_op = 0xa6,
    std_op = 0xa7,
    ldbu_op = 0xa8,
    ldhu_op = 0xa9,
    ldwu_op = 0xaa,
    flds_op = 0xac,
    fsts_op = 0xad,
    fldd_op = 0xae,
    fstd_op = 0xaf,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum reg2i16_op {
    jirl_op = 0x13,
    beq_op = 0x16,
    bne_op = 0x17,
    blt_op = 0x18,
    bge_op = 0x19,
    bltu_op = 0x1a,
    bgeu_op = 0x1b,
}

macro_rules! DEF_EMIT_REG1I20_FORMAT {
    ($name:ident, $opcode:expr) => {
        paste::paste! {
            fn [<emit_ $name>](rd: loongarch_gpr, imm: i32) -> u32 {
                let mut insn = reg1i20_format::new();
                insn.set_opcode(reg1i20_op::$opcode as u32);
                insn.set_rd(rd as u32);
                insn.set_immediate(imm as u32);
                insn.into()
            }
        }
    };
}

DEF_EMIT_REG1I20_FORMAT!(lu12iw, lu12iw_op);
DEF_EMIT_REG1I20_FORMAT!(lu32id, lu32id_op);
DEF_EMIT_REG1I20_FORMAT!(pcaddu18i, pcaddu18i_op);

macro_rules! DEF_EMIT_REG2I12_FORMAT {
    ($name:ident, $opcode:expr) => {
        paste::paste! {
            fn [<emit_ $name>](rd: loongarch_gpr, rj: loongarch_gpr, imm: i32) -> u32 {
                let mut insn = reg2i12_format::new();
                insn.set_opcode(reg2i12_op::$opcode as u32);
                insn.set_rd(rd as u32);
                insn.set_rj(rj as u32);
                insn.set_immediate(imm as u32);
                insn.into()
            }
        }
    };
}

DEF_EMIT_REG2I12_FORMAT!(addiw, addiw_op);
DEF_EMIT_REG2I12_FORMAT!(addid, addid_op);
DEF_EMIT_REG2I12_FORMAT!(lu52id, lu52id_op);
DEF_EMIT_REG2I12_FORMAT!(andi, andi_op);
DEF_EMIT_REG2I12_FORMAT!(ori, ori_op);
DEF_EMIT_REG2I12_FORMAT!(xori, xori_op);
DEF_EMIT_REG2I12_FORMAT!(ldbu, ldbu_op);
DEF_EMIT_REG2I12_FORMAT!(ldhu, ldhu_op);
DEF_EMIT_REG2I12_FORMAT!(ldwu, ldwu_op);
DEF_EMIT_REG2I12_FORMAT!(ldd, ldd_op);
DEF_EMIT_REG2I12_FORMAT!(stb, stb_op);
DEF_EMIT_REG2I12_FORMAT!(sth, sth_op);
DEF_EMIT_REG2I12_FORMAT!(stw, stw_op);
DEF_EMIT_REG2I12_FORMAT!(std, std_op);

macro_rules! DEF_EMIT_REG2I16_FORMAT {
    ($name:ident, $opcode:expr) => {
        paste::paste! {
            fn [<emit_ $name>](rd: loongarch_gpr, rj: loongarch_gpr, offset: i32) -> u32 {
                let mut insn = reg2i16_format::new();
                insn.set_opcode(reg2i16_op::$opcode as u32);
                insn.set_rd(rd as u32);
                insn.set_rj(rj as u32);
                insn.set_immediate(offset as u32);
                insn.into()
            }
        }
    };
}

DEF_EMIT_REG2I16_FORMAT!(beq, beq_op);
DEF_EMIT_REG2I16_FORMAT!(bne, bne_op);
DEF_EMIT_REG2I16_FORMAT!(blt, blt_op);
DEF_EMIT_REG2I16_FORMAT!(bge, bge_op);
DEF_EMIT_REG2I16_FORMAT!(bltu, bltu_op);
DEF_EMIT_REG2I16_FORMAT!(bgeu, bgeu_op);
DEF_EMIT_REG2I16_FORMAT!(jirl, jirl_op);

pub fn larch_insn_gen_lu12iw(rd: loongarch_gpr, imm: i32) -> u32 {
    if imm < -(SZ_512K as i32) || imm >= SZ_512K as i32 {
        log::warn!("The generated lu12i.w instruction is out of range.");
        return INSN_BREAK;
    }

    emit_lu12iw(rd, imm)
}

pub fn larch_insn_gen_lu32id(rd: loongarch_gpr, imm: i32) -> u32 {
    if imm < -(SZ_512K as i32) || imm >= SZ_512K as i32 {
        log::warn!("The generated lu32i.d instruction is out of range.");
        return INSN_BREAK;
    }
    emit_lu32id(rd, imm)
}

pub fn larch_insn_gen_lu52id(rd: loongarch_gpr, rj: loongarch_gpr, imm: i32) -> u32 {
    if imm < -(SZ_2K as i32) || imm >= SZ_2K as i32 {
        log::warn!("The generated lu52i.d instruction is out of range.");
        return INSN_BREAK;
    }
    emit_lu52id(rd, rj, imm)
}

pub fn larch_insn_gen_jirl(rd: loongarch_gpr, rj: loongarch_gpr, imm: i32) -> u32 {
    if (imm & 3) != 0 || imm < -(SZ_128K as i32) || imm >= SZ_128K as i32 {
        log::warn!("The generated jirl instruction is out of range.");
        return INSN_BREAK;
    }
    emit_jirl(rd, rj, imm >> 2)
}
