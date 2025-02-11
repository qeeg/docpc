/*
    MartyPC
    https://github.com/dbalsom/martypc

    Copyright 2022-2024 Daniel Balsom

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the “Software”),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.

    --------------------------------------------------------------------------

    devices::mda::io.rs

    Implementation of the IoDevice interface trait for the IBM MDA card.

*/

use super::*;
use crate::bus::{IoDevice, NO_IO_BYTE};

pub const LPT_DEFAULT_IO_BASE: u16 = 0x3BC;
pub const LPT_PORT_MASK: u16 = !0x003;

// CRTC registers are mirrored from 0x3D0 - 0x3D5 due to incomplete
// address decoding.
pub const CRTC_REGISTER_SELECT0: u16 = 0x3B0;
pub const CRTC_REGISTER_SELECT1: u16 = 0x3B2;
pub const CRTC_REGISTER_SELECT2: u16 = 0x3B4;
pub const CRTC_REGISTER_SELECT3: u16 = 0x3B6;

pub const CRTC_REGISTER0: u16 = 0x3B1;
pub const CRTC_REGISTER1: u16 = 0x3B3;
pub const CRTC_REGISTER2: u16 = 0x3B5;
pub const CRTC_REGISTER3: u16 = 0x3B7;

pub const CRTC_REGISTER_BASE: u16 = 0x3B0;
pub const CRTC_REGISTER_MASK: u16 = !0x007;

pub const MDA_MODE_CONTROL_REGISTER: u16 = 0x3B8;
pub const MDA_STATUS_REGISTER: u16 = 0x3BA;
//pub const CGA_LIGHTPEN_LATCH_RESET: u16 = 0x3DB;
//pub const CGA_LIGHTPEN_LATCH_SET: u16 = 0x3DC;

impl IoDevice for MDACard {
    fn read_u8(&mut self, port: u16, _delta: DeviceRunTimeUnit) -> u8 {
        // Catch up to CPU state.
        //let _ticks = self.catch_up(delta, false);

        //self.rw_op(ticks, 0, port as u32, RwSlotType::Io);

        if (port & CRTC_REGISTER_MASK) == CRTC_REGISTER_BASE {
            // Read is from CRTC register.
            self.crtc.port_read(port)
        }
        else if (port & LPT_PORT_MASK) == LPT_DEFAULT_IO_BASE {
            // Read is from LPT port.
            if let Some(lpt) = &mut self.lpt {
                log::warn!("Reading LPT of MDA card!");
                lpt.port_read(port)
            }
            else {
                NO_IO_BYTE
            }
        }
        else {
            // Must be some other MDA register
            match port {
                MDA_MODE_CONTROL_REGISTER => {
                    log::error!("CGA: Read from Mode control register!");
                    NO_IO_BYTE
                }
                MDA_STATUS_REGISTER => self.handle_status_register_read(),
                _ => NO_IO_BYTE,
            }
        }
    }

    fn write_u8(&mut self, port: u16, data: u8, _bus: Option<&mut BusInterface>, _delta: DeviceRunTimeUnit) {
        let _debug_port = (if port == 0x3D5 { true } else { false }) && self.debug;

        // Catch up to CPU state.
        //let _ticks = self.catch_up(delta, debug_port);

        //self.rw_op(ticks, data, port as u32, RwSlotType::Io);

        if (port & CRTC_REGISTER_MASK) == CRTC_REGISTER_BASE {
            // Write is to CRTC register.
            self.crtc.port_write(port, data);
        }
        else if (port & LPT_PORT_MASK) == LPT_DEFAULT_IO_BASE {
            // Read is from LPT port.
            if let Some(lpt) = &mut self.lpt {
                lpt.port_write(port, data);
            }
        }
        else {
            // Must be some other MDA register
            match port {
                MDA_MODE_CONTROL_REGISTER => {
                    self.handle_mode_register(data);
                }
                _ => {}
            }
        }
    }

    fn port_list(&self) -> Vec<u16> {
        let mut mda_ports = vec![
            CRTC_REGISTER_SELECT0,
            CRTC_REGISTER_SELECT1,
            CRTC_REGISTER_SELECT2,
            CRTC_REGISTER_SELECT3,
            CRTC_REGISTER0,
            CRTC_REGISTER1,
            CRTC_REGISTER2,
            CRTC_REGISTER3,
            MDA_MODE_CONTROL_REGISTER,
            MDA_STATUS_REGISTER,
        ];

        if self.lpt.is_some() {
            log::debug!("Adding LPT ports to MDA port list");
            mda_ports.extend([self.lpt_port_base, self.lpt_port_base + 1, self.lpt_port_base + 2].iter());
        }
        mda_ports
    }
}
